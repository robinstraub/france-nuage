use controlplane_core::claims::Claims;
use controlplane_core::error::Error;
use jsonwebtoken::{DecodingKey, TokenData, Validation, decode, jwk::JwkSet};
use moka::future::Cache;
use serde::Deserialize;
use std::time::Duration;

const JWK_CACHE_MAX_CAPACITY: u64 = 200;
const JWK_CACHE_TTL: u64 = 3600;

#[derive(Clone)]
pub struct OpenID {
    client: reqwest::Client,
    config: OpenIDProviderConfiguration,
    keys: Cache<String, DecodingKey>,
}

impl OpenID {
    pub async fn discover(client: reqwest::Client, url: &str) -> Result<Self, Error> {
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|_| Error::Internal(format!("unreachable oidc provider: {url}")))?;

        let config: OpenIDProviderConfiguration = response
            .json()
            .await
            .map_err(|_| Error::Internal(format!("unparsable oidc metadata: {url}")))?;

        Ok(Self {
            client,
            config,
            keys: Cache::builder()
                .max_capacity(JWK_CACHE_MAX_CAPACITY)
                .time_to_live(Duration::from_secs(JWK_CACHE_TTL))
                .build(),
        })
    }

    async fn get_or_fetch_key(&self, kid: &str) -> Result<DecodingKey, Error> {
        if let Some(key) = self.keys.get(kid).await {
            return Ok(key);
        }

        let keys = self.fetch_keys().await?;
        for (kid, decoding_key) in keys {
            self.keys.insert(kid, decoding_key).await;
        }

        self.keys.get(kid).await.ok_or(Error::Unauthenticated)
    }

    async fn fetch_keys(&self) -> Result<Vec<(String, DecodingKey)>, Error> {
        let response = self
            .client
            .get(&self.config.jwks_uri)
            .send()
            .await
            .map_err(|_| Error::Internal(format!("unreachable jwks endpoint: {}", self.config.jwks_uri)))?;

        let jwks = response
            .json::<JwkSet>()
            .await
            .map_err(|_| Error::Internal(format!("unparsable jwks: {}", self.config.jwks_uri)))?
            .keys;

        let mut result = Vec::with_capacity(jwks.len());
        for jwk in jwks {
            let kid = jwk.common.key_id.clone().ok_or(Error::Unauthenticated)?;
            let decoding_key = DecodingKey::from_jwk(&jwk).map_err(|_| Error::Unauthenticated)?;
            result.push((kid, decoding_key));
        }
        Ok(result)
    }

    pub async fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, Error> {
        let header = jsonwebtoken::decode_header(token).map_err(|_| Error::Unauthenticated)?;
        let kid = header.kid.ok_or(Error::Unauthenticated)?;

        let decoding_key = self.get_or_fetch_key(&kid).await?;
        let mut validation = Validation::new(header.alg);
        validation.validate_aud = false;
        validation.validate_exp = false;

        decode(token, &decoding_key, &validation).map_err(|_| Error::Unauthenticated)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct OpenIDProviderConfiguration {
    #[serde(rename = "issuer")]
    pub _issuer: String,
    pub jwks_uri: String,
}

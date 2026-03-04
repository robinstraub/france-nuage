use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use controlplane_core::claims::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rand::{SeedableRng, rngs::StdRng};
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rsa::traits::PublicKeyParts;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::json;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

const MOCK_JWK_KID: &str = "mock-key-01";

static RSA_KEYS: OnceLock<(RsaPrivateKey, RsaPublicKey)> = OnceLock::new();

fn rsa_keys() -> &'static (RsaPrivateKey, RsaPublicKey) {
    RSA_KEYS.get_or_init(|| {
        let mut rng = StdRng::from_seed([42u8; 32]);
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        (private_key, public_key)
    })
}

pub fn create_test_token(sub: &str, email: &str) -> String {
    let (private_key, _) = rsa_keys();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("could not get system time")
        .as_secs();

    let claims = Claims {
        sub: Some(sub.to_owned()),
        email: Some(email.to_owned()),
        iat: Some(now),
        exp: Some(now + 3600),
        nbf: Some(now),
        iss: None,
    };

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(MOCK_JWK_KID.to_owned());

    let pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .expect("could not create pem");
    let encoding_key =
        EncodingKey::from_rsa_pem(pem.as_bytes()).expect("could not create encoding key");

    encode(&header, &claims, &encoding_key).expect("could not encode token")
}

pub fn create_test_token_without_kid(sub: &str, email: &str) -> String {
    let (private_key, _) = rsa_keys();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("could not get system time")
        .as_secs();

    let claims = Claims {
        sub: Some(sub.to_owned()),
        email: Some(email.to_owned()),
        iat: Some(now),
        exp: Some(now + 3600),
        nbf: Some(now),
        iss: None,
    };

    let header = Header::new(Algorithm::RS256);

    let pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .expect("could not create pem");
    let encoding_key =
        EncodingKey::from_rsa_pem(pem.as_bytes()).expect("could not create encoding key");

    encode(&header, &claims, &encoding_key).expect("could not encode token")
}

pub async fn register_oidc_mocks(server: &mut mockito::ServerGuard) {
    register_well_known(server, &format!("{}/oauth/discovery/keys", server.url()));
    register_jwks(server);
}

pub fn register_well_known(server: &mut mockito::ServerGuard, jwks_uri: &str) {
    let base = server.url();
    server
        .mock("GET", "/.well-known/openid-configuration")
        .with_body(
            json!({
                "issuer": base,
                "jwks_uri": jwks_uri,
            })
            .to_string(),
        )
        .create();
}

pub fn register_well_known_with_invalid_body(server: &mut mockito::ServerGuard) {
    server
        .mock("GET", "/.well-known/openid-configuration")
        .with_body("not json")
        .create();
}

pub fn register_jwks(server: &mut mockito::ServerGuard) {
    let (_, public_key) = rsa_keys();
    let jwk = json!({
        "kty": "RSA",
        "use": "sig",
        "kid": MOCK_JWK_KID,
        "alg": "RS256",
        "n": URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be()),
        "e": URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be()),
    });

    server
        .mock("GET", "/oauth/discovery/keys")
        .with_body(json!({ "keys": [jwk] }).to_string())
        .create();
}

pub fn register_jwks_with_invalid_body(server: &mut mockito::ServerGuard) {
    server
        .mock("GET", "/oauth/discovery/keys")
        .with_body("not json")
        .create();
}

pub fn register_jwks_with_kidless_key(server: &mut mockito::ServerGuard) {
    let (_, public_key) = rsa_keys();
    let jwk = json!({
        "kty": "RSA",
        "use": "sig",
        "alg": "RS256",
        "n": URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be()),
        "e": URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be()),
    });

    server
        .mock("GET", "/oauth/discovery/keys")
        .with_body(json!({ "keys": [jwk] }).to_string())
        .create();
}

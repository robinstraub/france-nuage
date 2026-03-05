pub mod openid;

#[cfg(test)]
pub(crate) mod mock;

use controlplane_core::error::Error;
use domain::entities::User;
use domain::ports::UserRepository;
use tonic::Request;

use self::openid::OpenID;

pub fn extract_authorization_token<T>(request: &Request<T>) -> Result<String, Error> {
    request
        .metadata()
        .get("authorization")
        .ok_or(Error::Unauthenticated)?
        .to_str()
        .map_err(|_| Error::Unauthenticated)?
        .strip_prefix("Bearer ")
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .ok_or(Error::Unauthenticated)
}

pub async fn authenticate<T>(
    request: &Request<T>,
    openid: &OpenID,
    user_repo: &dyn UserRepository,
) -> Result<User, Error> {
    let token = extract_authorization_token(request)?;
    let token_data = openid.validate_token(&token).await?;

    let sub = token_data.claims.sub.ok_or(Error::Unauthenticated)?;
    let email = token_data.claims.email.ok_or(Error::Unauthenticated)?;

    user_repo.find_or_create(&sub, &email).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use mock::{
        create_test_token, create_test_token_without_kid, register_jwks_with_invalid_body,
        register_jwks_with_kidless_key, register_oidc_mocks, register_well_known,
        register_well_known_with_invalid_body,
    };
    use uuid::Uuid;

    struct MockUserRepository {
        existing_user: Option<User>,
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, Error> {
            Ok(self.existing_user.clone())
        }

        async fn create(&self, user: &User) -> Result<User, Error> {
            Ok(user.clone())
        }

        async fn find_or_create(&self, keycloak_id: &str, email: &str) -> Result<User, Error> {
            if let Some(user) = self.find_by_keycloak_id(keycloak_id).await? {
                return Ok(user);
            }

            let user = User {
                id: Uuid::new_v4(),
                keycloak_id: keycloak_id.to_string(),
                email: email.to_string(),
                name: None,
                created_at: Utc::now(),
            };
            self.create(&user).await
        }
    }

    fn discover_url(server: &mockito::ServerGuard) -> String {
        format!("{}/.well-known/openid-configuration", server.url())
    }

    #[test]
    fn extraire_le_token_depuis_un_header_authorization_valide() {
        let mut request = Request::new(());
        request.metadata_mut().insert(
            "authorization",
            "Bearer my-jwt-token".parse().unwrap(),
        );

        let result = extract_authorization_token(&request);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my-jwt-token");
    }

    #[test]
    fn erreur_unauthenticated_sans_header_authorization() {
        let request = Request::new(());

        let result = extract_authorization_token(&request);

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[test]
    fn erreur_unauthenticated_avec_header_sans_prefix_bearer() {
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", "Basic abc123".parse().unwrap());

        let result = extract_authorization_token(&request);

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[test]
    fn erreur_unauthenticated_avec_bearer_vide() {
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", "Bearer ".parse().unwrap());

        let result = extract_authorization_token(&request);

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[tokio::test]
    async fn erreur_quand_le_provider_oidc_est_injoignable() {
        let result = OpenID::discover(
            reqwest::Client::new(),
            "https://unreachable.invalid/.well-known/openid-configuration",
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn erreur_quand_les_metadonnees_oidc_sont_invalides() {
        let mut server = mockito::Server::new_async().await;
        register_well_known_with_invalid_body(&mut server);

        let result = OpenID::discover(reqwest::Client::new(), &discover_url(&server)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn discovery_reussie_avec_un_serveur_mock() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let result = OpenID::discover(reqwest::Client::new(), &discover_url(&server)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn validation_token_valide() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("user-123", "robin@france-nuage.fr");
        let result = openid.validate_token(&token).await;

        assert!(result.is_ok());
        let claims = result.unwrap().claims;
        assert_eq!(claims.sub.unwrap(), "user-123");
        assert_eq!(claims.email.unwrap(), "robin@france-nuage.fr");

        // second call hits the cache
        let result = openid.validate_token(&token).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn validation_token_invalide() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let result = openid.validate_token("invalid-token").await;

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[tokio::test]
    async fn erreur_validation_avec_token_sans_kid() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token_without_kid("user-123", "robin@france-nuage.fr");
        let result = openid.validate_token(&token).await;

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[tokio::test]
    async fn erreur_validation_avec_jwks_injoignable() {
        let mut server = mockito::Server::new_async().await;
        register_well_known(
            &mut server,
            "https://unreachable.invalid/oauth/discovery/keys",
        );

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("user-123", "robin@france-nuage.fr");
        let result = openid.validate_token(&token).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn erreur_validation_avec_jwks_invalide() {
        let mut server = mockito::Server::new_async().await;
        let jwks_uri = format!("{}/oauth/discovery/keys", server.url());
        register_well_known(&mut server, &jwks_uri);
        register_jwks_with_invalid_body(&mut server);

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("user-123", "robin@france-nuage.fr");
        let result = openid.validate_token(&token).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn erreur_validation_avec_jwk_sans_kid() {
        let mut server = mockito::Server::new_async().await;
        let jwks_uri = format!("{}/oauth/discovery/keys", server.url());
        register_well_known(&mut server, &jwks_uri);
        register_jwks_with_kidless_key(&mut server);

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("user-123", "robin@france-nuage.fr");
        let result = openid.validate_token(&token).await;

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }

    #[tokio::test]
    async fn authentification_cree_un_utilisateur_au_premier_login() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("keycloak-id-new", "new@france-nuage.fr");
        let mut request = Request::new(());
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}").parse().unwrap(),
        );

        let repo = MockUserRepository {
            existing_user: None,
        };

        let result = authenticate(&request, &openid, &repo).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.keycloak_id, "keycloak-id-new");
        assert_eq!(user.email, "new@france-nuage.fr");
    }

    #[tokio::test]
    async fn authentification_retrouve_un_utilisateur_existant() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let existing = User {
            id: Uuid::new_v4(),
            keycloak_id: "keycloak-id-existing".to_string(),
            email: "existing@france-nuage.fr".to_string(),
            name: Some("Robin".to_string()),
            created_at: Utc::now(),
        };

        let token = create_test_token("keycloak-id-existing", "existing@france-nuage.fr");
        let mut request = Request::new(());
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}").parse().unwrap(),
        );

        let repo = MockUserRepository {
            existing_user: Some(existing.clone()),
        };

        let result = authenticate(&request, &openid, &repo).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, existing.id);
        assert_eq!(user.keycloak_id, "keycloak-id-existing");
        assert_eq!(user.name, Some("Robin".to_string()));
    }

    #[tokio::test]
    async fn erreur_authenticate_sans_header_authorization() {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let request = Request::new(());
        let repo = MockUserRepository {
            existing_user: None,
        };

        let result = authenticate(&request, &openid, &repo).await;

        assert!(matches!(result.unwrap_err(), Error::Unauthenticated));
    }
}

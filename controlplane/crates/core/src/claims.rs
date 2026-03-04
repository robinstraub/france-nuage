use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Option<String>,
    pub email: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub iss: Option<String>,
    pub nbf: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_claims_se_deserialisent_depuis_un_json_valide() {
        let json = r#"{
            "sub": "user-123",
            "email": "robin@france-nuage.fr",
            "exp": 1700000000,
            "iat": 1699996400,
            "iss": "https://keycloak.localhost/realms/france-nuage",
            "nbf": 1699996400
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();

        assert_eq!(claims.sub.unwrap(), "user-123");
        assert_eq!(claims.email.unwrap(), "robin@france-nuage.fr");
        assert_eq!(claims.exp.unwrap(), 1700000000);
        assert_eq!(claims.iat.unwrap(), 1699996400);
        assert_eq!(
            claims.iss.unwrap(),
            "https://keycloak.localhost/realms/france-nuage"
        );
        assert_eq!(claims.nbf.unwrap(), 1699996400);
    }

    #[test]
    fn les_claims_se_deserialisent_avec_des_champs_optionnels_absents() {
        let json = r#"{}"#;

        let claims: Claims = serde_json::from_str(json).unwrap();

        assert!(claims.sub.is_none());
        assert!(claims.email.is_none());
        assert!(claims.exp.is_none());
        assert!(claims.iat.is_none());
        assert!(claims.iss.is_none());
        assert!(claims.nbf.is_none());
    }

    #[test]
    fn les_claims_se_serialisent_en_json() {
        let claims = Claims {
            sub: Some("user-456".into()),
            email: Some("test@example.com".into()),
            exp: Some(1700000000),
            iat: Some(1699996400),
            iss: None,
            nbf: None,
        };

        let json = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.sub.unwrap(), "user-456");
        assert_eq!(deserialized.email.unwrap(), "test@example.com");
        assert_eq!(deserialized.exp.unwrap(), 1700000000);
        assert!(deserialized.iss.is_none());
    }
}

use std::sync::Arc;

use domain::entities::Organization;
use domain::ports::{OrganizationRepository, UserRepository};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::auth::{authenticate, openid::OpenID};
use crate::proto::{
    resource_manager_service_server::ResourceManagerService, CreateOrganizationRequest,
    CreateOrganizationResponse, ListOrganizationsRequest, ListOrganizationsResponse,
    Organization as ProtoOrganization,
};

pub struct ResourceManagerHandler {
    openid: OpenID,
    user_repository: Arc<dyn UserRepository>,
    organization_repository: Arc<dyn OrganizationRepository>,
}

impl ResourceManagerHandler {
    pub fn new(
        openid: OpenID,
        user_repository: Arc<dyn UserRepository>,
        organization_repository: Arc<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            openid,
            user_repository,
            organization_repository,
        }
    }
}

#[tonic::async_trait]
impl ResourceManagerService for ResourceManagerHandler {
    async fn create_organization(
        &self,
        request: Request<CreateOrganizationRequest>,
    ) -> Result<Response<CreateOrganizationResponse>, Status> {
        let user =
            authenticate(&request, &self.openid, self.user_repository.as_ref()).await?;

        let name = request.into_inner().name.trim().to_string();
        if name.is_empty() {
            return Err(controlplane_core::error::Error::InvalidArgument(
                "name is required".into(),
            )
            .into());
        }

        let organization = Organization {
            id: Uuid::new_v4(),
            name,
            parent_id: None,
            created_at: chrono::Utc::now(),
        };

        let created = self
            .organization_repository
            .create(&organization, user.id)
            .await?;

        Ok(Response::new(CreateOrganizationResponse {
            organization: Some(ProtoOrganization {
                id: created.id.to_string(),
                name: created.name,
            }),
        }))
    }

    async fn list_organizations(
        &self,
        request: Request<ListOrganizationsRequest>,
    ) -> Result<Response<ListOrganizationsResponse>, Status> {
        let user =
            authenticate(&request, &self.openid, self.user_repository.as_ref()).await?;

        let organizations = self
            .organization_repository
            .list_by_user(user.id)
            .await?;

        let organizations = organizations
            .into_iter()
            .map(|org| ProtoOrganization {
                id: org.id.to_string(),
                name: org.name,
            })
            .collect();

        Ok(Response::new(ListOrganizationsResponse { organizations }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::mock::{create_test_token, register_oidc_mocks};
    use chrono::Utc;
    use controlplane_core::error::Error;
    use domain::entities::User;

    struct MockUserRepository;

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, Error> {
            Ok(None)
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

    struct MockOrganizationRepository;

    #[async_trait::async_trait]
    impl OrganizationRepository for MockOrganizationRepository {
        async fn create(
            &self,
            organization: &Organization,
            _user_id: Uuid,
        ) -> Result<Organization, Error> {
            Ok(organization.clone())
        }

        async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<Organization>, Error> {
            Ok(vec![
                Organization {
                    id: Uuid::new_v4(),
                    name: "Org A".to_string(),
                    parent_id: None,
                    created_at: Utc::now(),
                },
                Organization {
                    id: Uuid::new_v4(),
                    name: "Org B".to_string(),
                    parent_id: None,
                    created_at: Utc::now(),
                },
            ])
        }
    }

    struct FailingOrganizationRepository;

    #[async_trait::async_trait]
    impl OrganizationRepository for FailingOrganizationRepository {
        async fn create(
            &self,
            _organization: &Organization,
            _user_id: Uuid,
        ) -> Result<Organization, Error> {
            Err(Error::Internal("db error".into()))
        }

        async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<Organization>, Error> {
            Err(Error::Internal("db error".into()))
        }
    }

    fn discover_url(server: &mockito::ServerGuard) -> String {
        format!("{}/.well-known/openid-configuration", server.url())
    }

    async fn setup_handler(
        org_repo: Arc<dyn OrganizationRepository>,
    ) -> (ResourceManagerHandler, String, mockito::ServerGuard) {
        let mut server = mockito::Server::new_async().await;
        register_oidc_mocks(&mut server).await;

        let openid = OpenID::discover(reqwest::Client::new(), &discover_url(&server))
            .await
            .unwrap();

        let token = create_test_token("user-123", "test@france-nuage.fr");

        let handler = ResourceManagerHandler::new(
            openid,
            Arc::new(MockUserRepository),
            org_repo,
        );

        (handler, token, server)
    }

    fn authenticated_request<T>(token: &str, body: T) -> Request<T> {
        let mut request = Request::new(body);
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}").parse().unwrap(),
        );
        request
    }

    #[tokio::test]
    async fn creer_une_organisation_avec_succes() {
        let (handler, token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = authenticated_request(
            &token,
            CreateOrganizationRequest {
                name: "Mon Organisation".to_string(),
            },
        );

        let response = handler.create_organization(request).await;

        assert!(response.is_ok());
        let org = response.unwrap().into_inner().organization.unwrap();
        assert_eq!(org.name, "Mon Organisation");
        assert!(!org.id.is_empty());
    }

    #[tokio::test]
    async fn erreur_creation_organisation_avec_nom_vide() {
        let (handler, token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = authenticated_request(
            &token,
            CreateOrganizationRequest {
                name: "".to_string(),
            },
        );

        let response = handler.create_organization(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn erreur_creation_organisation_avec_nom_espaces() {
        let (handler, token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = authenticated_request(
            &token,
            CreateOrganizationRequest {
                name: "   ".to_string(),
            },
        );

        let response = handler.create_organization(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn creer_une_organisation_avec_espaces_autour_du_nom() {
        let (handler, token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = authenticated_request(
            &token,
            CreateOrganizationRequest {
                name: "  Mon Organisation  ".to_string(),
            },
        );

        let response = handler.create_organization(request).await;

        assert!(response.is_ok());
        let org = response.unwrap().into_inner().organization.unwrap();
        assert_eq!(org.name, "Mon Organisation");
    }

    #[tokio::test]
    async fn erreur_creation_organisation_sans_authentification() {
        let (handler, _token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = Request::new(CreateOrganizationRequest {
            name: "Test".to_string(),
        });

        let response = handler.create_organization(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::Unauthenticated);
    }

    #[tokio::test]
    async fn erreur_creation_organisation_quand_le_repo_echoue() {
        let (handler, token, _server) =
            setup_handler(Arc::new(FailingOrganizationRepository)).await;
        let request = authenticated_request(
            &token,
            CreateOrganizationRequest {
                name: "Test".to_string(),
            },
        );

        let response = handler.create_organization(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::Internal);
    }

    #[tokio::test]
    async fn lister_les_organisations_avec_succes() {
        let (handler, token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = authenticated_request(&token, ListOrganizationsRequest {});

        let response = handler.list_organizations(request).await;

        assert!(response.is_ok());
        let orgs = response.unwrap().into_inner().organizations;
        assert_eq!(orgs.len(), 2);
        assert_eq!(orgs[0].name, "Org A");
        assert_eq!(orgs[1].name, "Org B");
    }

    #[tokio::test]
    async fn erreur_lister_organisations_sans_authentification() {
        let (handler, _token, _server) = setup_handler(Arc::new(MockOrganizationRepository)).await;
        let request = Request::new(ListOrganizationsRequest {});

        let response = handler.list_organizations(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::Unauthenticated);
    }

    #[tokio::test]
    async fn erreur_lister_organisations_quand_le_repo_echoue() {
        let (handler, token, _server) =
            setup_handler(Arc::new(FailingOrganizationRepository)).await;
        let request = authenticated_request(&token, ListOrganizationsRequest {});

        let response = handler.list_organizations(request).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::Internal);
    }
}

use crate::entities::User;
use controlplane_core::error::Error;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, Error>;
    async fn create(&self, user: &User) -> Result<User, Error>;
    async fn find_or_create(&self, keycloak_id: &str, email: &str) -> Result<User, Error>;
}

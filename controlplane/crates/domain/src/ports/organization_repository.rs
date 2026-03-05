use crate::entities::Organization;
use controlplane_core::error::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn create(&self, organization: &Organization, user_id: Uuid) -> Result<Organization, Error>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Organization>, Error>;
}

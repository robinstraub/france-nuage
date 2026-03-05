use controlplane_core::error::Error;
use domain::entities::Organization;
use domain::ports::OrganizationRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgOrganizationRepository {
    pool: PgPool,
}

impl PgOrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OrganizationRepository for PgOrganizationRepository {
    async fn create(
        &self,
        organization: &Organization,
        user_id: Uuid,
    ) -> Result<Organization, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        let row = sqlx::query_as::<_, Organization>(
            "INSERT INTO organizations (id, name, parent_id, created_at) VALUES ($1, $2, $3, $4) RETURNING id, name, parent_id, created_at",
        )
        .bind(organization.id)
        .bind(&organization.name)
        .bind(organization.parent_id)
        .bind(organization.created_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO organization_user (organization_id, user_id) VALUES ($1, $2)",
        )
        .bind(organization.id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(row)
    }

    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Organization>, Error> {
        sqlx::query_as::<_, Organization>(
            "SELECT o.id, o.name, o.parent_id, o.created_at FROM organizations o INNER JOIN organization_user ou ON o.id = ou.organization_id WHERE ou.user_id = $1 ORDER BY o.created_at",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Internal(e.to_string()))
    }
}

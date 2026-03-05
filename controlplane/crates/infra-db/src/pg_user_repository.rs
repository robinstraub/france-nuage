use controlplane_core::error::Error;
use domain::entities::User;
use domain::ports::UserRepository;
use sqlx::PgPool;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, keycloak_id, email, name, created_at FROM users WHERE keycloak_id = $1",
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Internal(e.to_string()))
    }

    async fn create(&self, user: &User) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, keycloak_id, email, name, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id, keycloak_id, email, name, created_at",
        )
        .bind(user.id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Internal(e.to_string()))
    }

    async fn find_or_create(&self, keycloak_id: &str, email: &str) -> Result<User, Error> {
        if let Some(user) = self.find_by_keycloak_id(keycloak_id).await? {
            return Ok(user);
        }

        let user = User {
            id: uuid::Uuid::new_v4(),
            keycloak_id: keycloak_id.to_string(),
            email: email.to_string(),
            name: None,
            created_at: chrono::Utc::now(),
        };
        self.create(&user).await
    }
}

use controlplane_core::error::Error;
use domain::entities::User;
use domain::ports::UserRepository;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
struct UserRow {
    id: uuid::Uuid,
    keycloak_id: String,
    email: String,
    name: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            keycloak_id: row.keycloak_id,
            email: row.email,
            name: row.name,
            created_at: row.created_at,
        }
    }
}

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
        sqlx::query_as::<_, UserRow>(
            "SELECT id, keycloak_id, email, name, created_at FROM users WHERE keycloak_id = $1",
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await
        .map(|opt| opt.map(User::from))
        .map_err(|e| Error::Internal(e.to_string()))
    }

    async fn create(&self, user: &User) -> Result<User, Error> {
        sqlx::query_as::<_, UserRow>(
            "INSERT INTO users (id, keycloak_id, email, name, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id, keycloak_id, email, name, created_at",
        )
        .bind(user.id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await
        .map(User::from)
        .map_err(|e| Error::Internal(e.to_string()))
    }

    async fn find_or_create(&self, keycloak_id: &str, email: &str) -> Result<User, Error> {
        let user = User {
            id: uuid::Uuid::new_v4(),
            keycloak_id: keycloak_id.to_string(),
            email: email.to_string(),
            name: None,
            created_at: chrono::Utc::now(),
        };

        sqlx::query(
            "INSERT INTO users (id, keycloak_id, email, name, created_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (keycloak_id) DO NOTHING",
        )
        .bind(user.id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

        self.find_by_keycloak_id(keycloak_id)
            .await?
            .ok_or_else(|| Error::Internal("user not found after upsert".to_string()))
    }
}

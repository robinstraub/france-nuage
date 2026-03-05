use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

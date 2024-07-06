use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct Example {
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content: Option<String>,
}

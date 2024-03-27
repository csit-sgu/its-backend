use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct BookUpdate {
    pub title: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    #[serde(skip_deserializing, default = "Utc::now")]
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct AuthorUpdate {
    pub name: String,
}

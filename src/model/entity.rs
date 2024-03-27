use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Trait for database entities.
pub(crate) trait Entity {
    fn table_name() -> &'static str;
}

/// Quickly implement `Entity` trait on a struct.
macro_rules! impl_entity {
    ($sn:ident, $tn:literal) => {
        impl Entity for $sn {
            fn table_name() -> &'static str {
                $tn
            }
        }
    };
}

// NOTE(vinc3nzo): still too much code repetition.
// Need to implement proc-macro for this.

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Object)]
pub(crate) struct Book {
    pub book_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    #[serde(skip_serializing)]
    pub last_update: DateTime<Utc>,
}

impl_entity!(Book, "books");

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Object)]
pub(crate) struct Author {
    pub author_id: Uuid,
    pub name: String,
}

impl_entity!(Author, "authors");

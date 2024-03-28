use chrono::{DateTime, Utc};

/// Trait for database entities.
pub(crate) trait Entity {
    fn table_name() -> &'static str;
}

/// Quickly implement `Entity` trait on a struct.
#[allow(unused_macros)]
macro_rules! impl_entity {
    ($sn:ident, $tn:literal) => {
        impl Entity for $sn {
            fn table_name() -> &'static str {
                $tn
            }
        }
    };
}

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
    serde::Serialize,
    serde::Deserialize,
    poem_openapi::Object,
)]
pub struct AggregatedTask {
    pub task_id: u32,
    pub deadline: DateTime<Utc>,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub place_id: u32,
    pub lat: f32,
    pub lon: f32,
    pub region: String,
}

impl_entity!(AggregatedTask, "aggregated_tasks");

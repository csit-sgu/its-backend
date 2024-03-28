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
    pub account_id: u32,
    pub assigner_id: u32,
    pub taskable_type: String,
    pub deadline_at: DateTime<Utc>,
    pub object_id: u32,
    pub place_id: u32,
    pub region_id: u32,
    pub district_id: u32,
}

impl_entity!(AggregatedTask, "aggregated_tasks");

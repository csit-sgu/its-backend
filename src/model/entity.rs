use chrono::{DateTime, Utc};

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

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
    serde::Serialize,
    serde::Deserialize,
    poem_openapi::Object,
)]
pub struct FlatTask {
    pub task_id: u32,
    pub task_type: String,
    pub task_deadline: DateTime<Utc>,
    pub task_account_id: u32,
    pub task_created_at: DateTime<Utc>,
    pub task_transition_title: String,
    pub task_transitioned_at: DateTime<Utc>,
    pub task_stage_is_start: bool,
    pub task_stage_is_fulfilled: bool,
    pub task_stage_is_closed: bool,
    pub task_stage_is_cancelled: bool,
    pub object_id: u32,
    pub object_place_id: u32,
    pub place_lat: f32,
    pub place_lon: f32,
    pub region_title: String,
    pub region_id: u32,
}

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
    serde::Serialize,
    serde::Deserialize,
    poem_openapi::Object,
)]
pub struct TransitionView {
    id: u32,
    task_id: u32,
    transitioned_by: String,
    transitioned_at: DateTime<Utc>,
    stage_title: String,
}

impl_entity!(TransitionView, "transition_view");

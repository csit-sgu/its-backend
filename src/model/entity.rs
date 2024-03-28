use chrono::{DateTime, Utc};

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
    pub task_type: String,
    pub task_deadline: DateTime<Utc>,
    pub task_account_id: u32,
    pub task_created_at: DateTime<Utc>,
    pub task_transition_title: String,
    pub task_transitioned_at: DateTime<Utc>,
    pub object_id: u32,
    pub object_place_id: u32,
    pub place_lat: f32,
    pub place_lon: f32,
    pub region_title: String,
    pub region_id: u32,
}

// #[derive(
//     Debug,
//     Clone,
//     sqlx::FromRow,
//     serde::Serialize,
//     serde::Deserialize,
//     poem_openapi::Object,
// )]
// pub struct DetailedTask {
//     pub task_id: u32,
//     pub task_type: String,
//     pub task_description: Option<String>,
//     pub deadline: DateTime<Utc>,
//     pub account_id: u32,
//     pub account_name: String,
//     pub assigner_id: u32,
//     pub created: DateTime<Utc>,
//     pub status: String,
//     pub transition_status: String,
//     pub transition_timestamp: String,
//     pub place_id: u32,
//     pub object_id: u32,
//     pub lat: f32,
//     pub lon: f32,
//     pub region: String,
//     pub region_id: u32,
// }


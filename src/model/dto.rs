// define your DTOs here
// helpful derive macros:
// Debug, Clone, serde::{Serialize, Deserialize}, poem_openapi::Object

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Location {
    lat: f32,
    lon: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transition {
    status: String, // task_transitions.task_stage_id -> task_stages.title
    timestamp: DateTime<Utc>, // task_transitions.transitioned_at
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceObject {
    place_id: u32,      // places.id
    location: Location, // places.location
    region: String,     // regions.title
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskType {
    Regular,
    Incident,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    transitions: Vec<Transition>,
    obj: ServiceObject,
    deadline: DateTime<Utc>,
}

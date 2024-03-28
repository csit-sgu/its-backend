use chrono::{DateTime, Utc};
use derive_more::Display;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Display, Clone, Serialize, Deserialize, Enum)]
enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct User {
    pub user_id: usize,
    pub username: String,
    pub password: String,
    pub role: UserRole,
}

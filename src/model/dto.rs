use chrono::{DateTime, Utc};
use derive_more::Display;
use poem_openapi::{Enum, Object};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    Regular,
    Incident,
}

impl TryFrom<&str> for TaskType {
    type Error = &'static str;
    fn try_from(o: &str) -> Result<Self, Self::Error> {
        match o {
            "regular" => Ok(TaskType::Regular),
            "incident" => Ok(TaskType::Incident),
            _ => Err("wrong type of task"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    transitions: Vec<Transition>,
    obj: ServiceObject,
    deadline: DateTime<Utc>,
    task_type: TaskType,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize, Enum)]
pub enum UserRole {
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

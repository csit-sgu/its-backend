use chrono::{DateTime, Utc};
use derive_more::Display;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};

use super::entity::AggregatedTask;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub status: String, // task_transitions.task_stage_id -> task_stages.title
    pub timestamp: DateTime<Utc>, // task_transitions.transitioned_at
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceObject {
    pub place_id: u32,      // places.id
    pub location: Location, // places.location
    pub region: String,     // place_id -> places.id -> places.title
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum TaskType {
    // #[serde(rename(deserialize = "App\\Models\\ServiceDesk\\Regular"))]
    Regular,
    // #[serde(rename(deserialize = "App\\Models\\ServiceDesk\\Incident"))]
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
pub struct Task {
    pub task_id: u32,
    pub transitions: Vec<Transition>,
    pub obj: ServiceObject,
    pub deadline: DateTime<Utc>,
    pub task_type: TaskType,
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

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct AggregatedTasksResp {
    pub total_pages: usize,
    pub data: Vec<AggregatedTask>,
}

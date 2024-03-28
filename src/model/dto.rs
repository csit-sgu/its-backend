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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Regular,
    Incident,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: u32,
    pub transitions: Vec<Transition>,
    pub obj: ServiceObject,
    pub deadline: DateTime<Utc>,
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

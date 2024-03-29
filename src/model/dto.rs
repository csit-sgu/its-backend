use chrono::{DateTime, Utc};
use derive_more::Display;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct StageInfo {
    pub is_start: bool,
    pub is_fulfilled: bool,
    pub is_closed: bool,
    pub is_cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct Transition {
    pub status: String,
    pub transitioned_at: DateTime<Utc>,
    pub stage_info: StageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct ServiceObject {
    pub object_id: u32,
    pub object_place_id: u32,
    pub location: Location,
    pub region_id: u32,
    pub region_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, Enum)]
pub enum TaskType {
    #[oai(rename = "regular")]
    #[display(fmt = "App\\Models\\ServiceDesk\\Regular")]
    Regular,
    #[oai(rename = "incident")]
    #[display(fmt = "App\\Models\\ServiceDesk\\Incident")]
    Incident,
}

impl TryFrom<&str> for TaskType {
    type Error = &'static str;

    fn try_from(o: &str) -> Result<Self, Self::Error> {
        match o {
            "regular" => Ok(TaskType::Regular),
            "incident" => Ok(TaskType::Incident),
            "App\\Models\\ServiceDesk\\Incident" => Ok(TaskType::Incident),
            "App\\Models\\ServiceDesk\\Regular" => Ok(TaskType::Regular),
            _ => Err("wrong type of task"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct Task {
    pub task_id: u32,
    pub task_type: TaskType,
    pub transitions: Vec<Transition>,
    pub object: ServiceObject,
    pub deadline_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct Account {
    pub account_id: u32,
    pub title: String,
    pub account_type_id: u32,
    pub account_type_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct DetailedServiceObject {
    pub object_id: u32,
    pub object_place_id: u32,
    pub object_title: String,
    pub object_subtitle: Option<String>,
    pub location: Location,
    pub region_id: u32,
    pub region_title: String,
    pub region_capital: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct DetailedTransition {
    pub status: String,
    pub transitioned_at: DateTime<Utc>,
    pub stage_info: StageInfo,
    pub transitioned_by_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct DetailedTask {
    pub task_id: u32,
    pub task_type: TaskType,
    pub deadline_at: DateTime<Utc>,
    pub description: Option<String>,
    pub account: Account,
    pub transitions: Vec<DetailedTransition>,
    pub object: DetailedServiceObject,
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
    pub data: Vec<Task>,
}

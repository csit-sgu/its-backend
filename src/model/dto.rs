// define your DTOs here
// helpful derive macros:
// Debug, Clone, serde::{Serialize, Deserialize}, poem_openapi::Object
//
//
//
use derive_more::Display;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};

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

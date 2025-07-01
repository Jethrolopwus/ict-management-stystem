use serde::{Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(AsChangeset, serde::Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
}

// Optionally, if you want to serialize User for API responses, define a DTO:
#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role,
            active: user.active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
} 
use bson::DateTime;
use serde::{Deserialize, Serialize};

// Structure for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub name: String,
    pub lastname: String,
    pub major: String,
    pub email: String,
    pub password: String,
    pub courses: Vec<String>,
    pub created_at: DateTime,
    pub updated_st: DateTime,
}
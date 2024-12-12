use bson::DateTime;
use serde::{Deserialize, Serialize};

// Structure for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub title: String,
    pub company: String,
    pub author: String,
    pub finished_at: Option<DateTime>,
    pub duration: i32,
    pub language: String,
    pub description: String,
    pub url: String,
    pub topics: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
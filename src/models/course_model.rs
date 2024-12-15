use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

// Structure for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id         : Option<ObjectId>, // Optional field
    pub title       : String,
    pub platform    : String,
    pub author      : String,
    pub duration    : i32,
    pub language    : String,
    pub description : String,
    pub url         : String,
    pub topics      : Vec<String>,
    pub created_at  : DateTime,
    pub updated_at  : DateTime,
}
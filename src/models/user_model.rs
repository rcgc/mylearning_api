use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

// Structure for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id         : Option<ObjectId>,
    pub name        : String,
    pub lastname    : String,
    pub major       : String,
    pub email       : String,
    pub password    : String,
    pub courses     : Option<Vec<String>>,
    pub created_at  : DateTime,
    pub updated_at  : DateTime,
}
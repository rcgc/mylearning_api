use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Watched {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id         : Option<ObjectId>,
    pub course_id   : ObjectId,
    pub finished_at : DateTime,
    pub created_at  : DateTime,
    pub updated_at  : DateTime,
    pub archived    : bool,
}
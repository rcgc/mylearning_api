use crate::models::user_model::User;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error as MongoError,
    results::{DeleteResult, UpdateResult, InsertOneResult},
    Collection,
};
use futures::stream::StreamExt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiServiceError {
    #[error("Invalid ObjectId format")]
    InvalidObjectId,
    #[error("Database error: {0}")]
    DatabaseError(#[from] MongoError),
}

#[derive(Clone)]
pub struct ApiService {
    collection: Collection<User>,
}

// Helper function to convert a `Course` into a MongoDB Document.
fn user_to_document(u: &User) -> Document {
    doc! {
        "name"          : u.name.clone(),
        "lastname"      : u.lastname.clone(),
        "major"         : u.major.clone(),
        "email"         : u.email.clone(),
        "password"      : u.password.clone(),
        "courses"       : u.courses.clone().unwrap_or_default(), // Use an empty Vec if None
        "created_at"    : u.created_at,
        "updated_at"    : u.updated_at,
    }
}

impl ApiService {
    pub fn new(collection: Collection<User>) -> ApiService {
        ApiService { collection }
    }

    /// Get all courses from the collection.
    pub async fn get_all(&self) -> Result<Vec<User>, MongoError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut docs = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(course) => docs.push(course),
                Err(err) => return Err(err),
            }
        }

        Ok(docs)
    }

    /// Get a course by its MongoDB `_id`.
    pub async fn get_by_id(&self, course_id: &str) -> Result<Option<User>, ApiServiceError> {
        let object_id = ObjectId::parse_str(course_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    /// Create a new course in the collection.
    pub async fn create(&self, u: &User) -> Result<InsertOneResult, MongoError> {
        self.collection.insert_one(u, None).await
    }

    /// Update an existing course by its MongoDB `_id`.
    pub async fn update(&self, c: &User, user_id: &str) -> Result<UpdateResult, ApiServiceError> {
        let object_id = ObjectId::parse_str(user_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": user_to_document(c) };
        let result = self.collection.update_one(filter, update, None).await?;
        Ok(result)
    }

    /// Delete a course by its MongoDB `_id`.
    pub async fn delete(&self, user_id: &str) -> Result<DeleteResult, ApiServiceError> {
        let object_id = ObjectId::parse_str(user_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result)
    }
}
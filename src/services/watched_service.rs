use crate::models::watched_model::Watched;
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
    collection: Collection<Watched>,
}

// Helper function to convert a `Watched` into a MongoDB Document.
fn watched_to_document(w: &Watched) -> Document {
    doc! {
        "course_id"    : w.course_id.clone(),
        "finished_at"  : w.finished_at,
        "created_at"   : w.created_at,
        "updated_at"   : w.updated_at,
        "archived"     : w.archived,
    }
}

impl ApiService {
    pub fn new(collection: Collection<Watched>) -> ApiService {
        ApiService { collection }
    }

    pub async fn get_all(&self) -> Result<Vec<Watched>, MongoError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut docs = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(watched) => docs.push(watched),
                Err(err) => return Err(err),
            }
        }

        Ok(docs)
    }

    pub async fn get_by_id(&self, watched_id: &str) -> Result<Option<Watched>, ApiServiceError> {
        let object_id = ObjectId::parse_str(watched_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let result = self.collection.find_one(filter, None).await?;
        Ok(result)
    }

    pub async fn create(&self, w: &Watched) -> Result<InsertOneResult, MongoError> {
        self.collection.insert_one(w, None).await
    }

    pub async fn update(&self, c: &Watched, watched_id: &str) -> Result<UpdateResult, ApiServiceError> {
        let object_id = ObjectId::parse_str(watched_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": watched_to_document(c) };
        let result = self.collection.update_one(filter, update, None).await?;
        Ok(result)
    }

    pub async fn delete(&self, watched_id: &str) -> Result<DeleteResult, ApiServiceError> {
        let object_id = ObjectId::parse_str(watched_id).map_err(|_| ApiServiceError::InvalidObjectId)?;
        let filter = doc! { "_id": object_id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result)
    }
}

use crate::models::user_model::User; // Assuming `User` model exists
use mongodb::{bson::Document, error::Error as MongoError};
use futures::stream::StreamExt;

#[derive(Clone)]
pub struct ApiService {
    collection: mongodb::Collection<User>,
}

impl ApiService {
    pub fn new(collection: mongodb::Collection<User>) -> Self {
        Self { collection }
    }

    /// Search for a user by email.
    pub async fn search(&self, filter: Document) -> Result<Vec<User>, MongoError> {
        let mut cursor = self.collection.find(filter, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(user) => results.push(user),
                Err(err) => return Err(err),
            }
        }

        Ok(results)
    }
}

use crate::models::course_model::Course;
use mongodb::{bson::Document, error::Error as MongoError};
use futures::stream::StreamExt;

#[derive(Clone)]
pub struct ApiService {
    collection: mongodb::Collection<Course>,
}

impl ApiService {
    pub fn new(collection: mongodb::Collection<Course>) -> Self {
        Self { collection }
    }

    /// Search for courses by title, author, or platform with case-insensitive matching.
    /// At least one of the fields must match.
    pub async fn search(&self, filter: Document) -> Result<Vec<Course>, MongoError> {
        let mut cursor = self.collection.find(filter, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(course) => results.push(course),
                Err(err) => return Err(err),
            }
        }

        Ok(results)
    }
}

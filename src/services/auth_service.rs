use crate::models::{auth_model::LoginRequest, user_model::User};
use mongodb::{bson::doc, error::Error as MongoError, Collection};
use bcrypt::verify;

#[derive(Clone)]
pub struct ApiService {
    collection: Collection<User>,
}

impl ApiService {
    pub fn new(collection: Collection<User>) -> ApiService {
        ApiService { collection }
    }

    /// Authenticate a user using email and password.
    pub async fn login(&self, credentials: &LoginRequest) -> Result<Option<User>, MongoError> {
        // Find the user by email
        if let Some(user) = self
            .collection
            .find_one(doc! { "email": &credentials.email }, None)
            .await?
        {
            // Verify the provided password against the hashed password
            if verify(&credentials.password, &user.password).unwrap_or(false) {
                return Ok(Some(user)); // Return the user object on success
            }
        }
        Ok(None) // Return None if credentials are invalid
    }
}

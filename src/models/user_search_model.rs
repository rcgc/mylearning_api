use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserSearchParams {
    pub email: Option<String>, // The email field for searching users
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CourseSearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
    pub platform: Option<String>,
    pub topics: Option<Vec<String>>, // Topics is an optional array of strings
}
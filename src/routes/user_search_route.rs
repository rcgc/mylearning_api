use crate::models::user_search_model::UserSearchParams;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::doc;

#[post("/users/search")]
async fn search_users(
    app_data: web::Data<crate::AppState>, // AppState to access services
    body: web::Json<UserSearchParams>,    // Request body for email search
) -> impl Responder {
    // Build the filter only if email is provided
    if let Some(ref email) = body.email {
        let filter = doc! { "email": { "$regex": email, "$options": "i" } };

        // Perform the search in the database
        match app_data.service_manager.user_search_service.search(filter).await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(e) => {
                eprintln!("Error while searching users: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to search users")
            }
        }
    } else {
        HttpResponse::BadRequest().body("Email field must be provided")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(search_users);
}

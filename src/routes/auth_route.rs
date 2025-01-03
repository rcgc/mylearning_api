use crate::models::auth_model::LoginRequest;
use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;

#[post("/auth/login")]
async fn login(
    app_data: web::Data<AppState>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    // Attempt to authenticate the user
    let result = app_data
        .service_manager
        .auth_service
        .login(&credentials)
        .await;

    match result {
        Ok(Some(user)) => {
            // Get the current timestamp in milliseconds and convert to string
            let login_time = chrono::Utc::now().timestamp_millis().to_string();

            // Build the response JSON
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Login successful",
                "login_time": login_time,  // Include the Unix timestamp as a string
                "user": {
                    "name": user.name,
                    "lastname": user.lastname,
                    "email": user.email,
                    "major": user.major,
                    "courses": user.courses,
                    "created_at": user.created_at,
                    "updated_at": user.updated_at,
                }
            }))
        },
        Ok(None) => HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(_) => HttpResponse::InternalServerError().body("Error occurred while processing login"),
    }
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

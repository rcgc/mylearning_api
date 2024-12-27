use actix_web::{get, web, HttpResponse, Responder};

/// Health check route to verify the API is running
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("mylearning API is running")
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(health); // Health enpoint
}
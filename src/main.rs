mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use services::course_service::ApiService;
use routes::course_route;

pub struct ServiceManager {
    api: ApiService,
}

impl ServiceManager {
    pub fn new(api: ApiService) -> Self {
        ServiceManager { api }
    }
}

pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not in .env file");
    let client_options = ClientOptions::parse(database_url).await?;

    let client = Client::with_options(client_options)?;

    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not in .env file");
    let db = client.database(&database_name);

    let course_collection_name = env::var("COURSE_COLLECTION_NAME").expect("COURSE_COLLECTION_NAME is not in .env file");
    let course_collection = db.collection(&course_collection_name);

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not in .env file");

    HttpServer::new(move || {
        let course_service_worker = ApiService::new(course_collection.clone());
        let service_manager = ServiceManager::new(course_service_worker);

        let cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(AppState { service_manager }))
            .configure(course_route::init)
    })
    .bind(server_url)?
    .run()
    .await?;

    Ok(())
}

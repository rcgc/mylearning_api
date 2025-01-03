mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use services::{
    auth_service::ApiService as AuthService,
    course_service::ApiService as CourseService,
    user_service::ApiService as UserService,
    watched_service::ApiService as WatchedService
};
use routes::{ auth_route, course_route, health_route, user_route, watched_route };

#[derive(Clone)]
pub struct ServiceManager {
    pub auth_service: AuthService,
    pub course_service: CourseService,
    pub user_service: UserService,
    pub watched_service: WatchedService,
}

impl ServiceManager {
    pub fn new(auth_service:AuthService, course_service: CourseService, user_service: UserService, watched_service: WatchedService) -> Self {
        ServiceManager {
            auth_service,
            course_service,
            user_service,
            watched_service,
        }
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

    let auth_collection_name = env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let course_collection_name = env::var("COURSE_COLLECTION_NAME").expect("COURSE_COLLECTION_NAME is not set in .env file");
    let user_collection_name = env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let watched_collection_name = env::var("WATCHED_COLLECTION_NAME").expect("WATCHED_COLLECTION_NAME is not set in .env file");

    let auth_collection = db.collection(&auth_collection_name);
    let course_collection = db.collection(&course_collection_name);
    let user_collection = db.collection(&user_collection_name);
    let watched_collection = db.collection(&watched_collection_name);

    let auth_service = AuthService::new(auth_collection);
    let course_service = CourseService::new(course_collection);
    let user_service = UserService::new(user_collection);
    let watched_service = WatchedService::new(watched_collection);

    let service_manager = ServiceManager::new(auth_service, course_service, user_service, watched_service);

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    HttpServer::new(move || {
        let cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(AppState {
                service_manager: service_manager.clone(),
            }))
            .configure(auth_route::init)
            .configure(course_route::init)
            .configure(user_route::init)
            .configure(watched_route::init)
            .configure(health_route::init)
    })
    .bind(server_url)?
    .run()
    .await?;

    Ok(())
}

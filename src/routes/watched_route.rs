use crate::models::watched_model::Watched;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/watched")]
async fn get_all(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_manager.watched_service.get_all().await;
    match result {
        Ok(watcheds) => HttpResponse::Ok().json(watcheds),
        Err(e) => {
            eprintln!("Error while getting watcheds: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve watcheds")
        }
    }
}

#[get("/watched/{id}")]
async fn get_by_id(
    app_data: web::Data<crate::AppState>,
    watched_id: web::Path<String>,
) -> impl Responder {
    let id = watched_id.into_inner(); // Extract `id` as a String
    match app_data.service_manager.watched_service.get_by_id(&id).await {
        Ok(Some(watched)) => HttpResponse::Ok().json(watched), 
        Ok(None) => HttpResponse::NotFound().body("Watched not found"),
        Err(e) => {
            eprintln!("Error while getting watched: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve the watched")
        }
    }
}


#[post("/watched")]
async fn add(app_data: web::Data<crate::AppState>, data: web::Json<Watched>) -> impl Responder {
    match app_data.service_manager.watched_service.create(&data).await {
        Ok(result) => match result.inserted_id.as_object_id() {
            Some(id) => HttpResponse::Ok().json(id.to_hex()),
            None => HttpResponse::InternalServerError().body("Failed to extract inserted_id"),
        },
        Err(e) => {
            eprintln!("Error while adding watched: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add the watched")
        }
    }
}

#[put("/watched/{id}")]
async fn update(
    app_data: web::Data<crate::AppState>,
    data: web::Json<Watched>,
    watched_id: web::Path<String>,
) -> impl Responder {
    let id = watched_id.into_inner();
    match app_data.service_manager.watched_service.update(&data, &id).await {
        Ok(result) => {
            if result.modified_count > 0 {
                HttpResponse::Ok().json("Watched updated successfully")
            } else {
                HttpResponse::NotFound().body("Watched not found or no changes made")
            }
        }
        Err(e) => {
            eprintln!("Error while updating watched: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update the watched")
        }
    }
}

#[delete("/watched/{id}")]
async fn delete(
    app_data: web::Data<crate::AppState>,
    watched_id: web::Path<String>,
) -> impl Responder {
    let id = watched_id.into_inner();
    match app_data.service_manager.watched_service.delete(&id).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().json("Watched deleted successfully")
            } else {
                HttpResponse::NotFound().body("Watched not found")
            }
        }
        Err(e) => {
            eprintln!("Error while deleting watched: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete the watched")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(add);
    cfg.service(update);
    cfg.service(delete);
}
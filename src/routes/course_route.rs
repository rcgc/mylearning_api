use crate::models::course_model::Course;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

/// Route to get all courses
#[get("/courses/get-all")]
async fn get_all(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_manager.course_service.get_all().await;
    match result {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            eprintln!("Error while getting courses: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve courses")
        }
    }
}

/// Route to get a course by its MongoDB `_id`
#[get("/courses/get-by-id/{id}")]
async fn get_by_id(
    app_data: web::Data<crate::AppState>,
    course_id: web::Path<String>,
) -> impl Responder {
    let id = course_id.into_inner(); // Extract `id` as a String
    match app_data.service_manager.course_service.get_by_id(&id).await {
        Ok(Some(course)) => HttpResponse::Ok().json(course), // Course found
        Ok(None) => HttpResponse::NotFound().body("Course not found"), // Course not found
        Err(e) => {
            eprintln!("Error while getting course: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve the course")
        }
    }
}

/// Route to add a new course
#[post("/courses/add")]
async fn add(app_data: web::Data<crate::AppState>, data: web::Json<Course>) -> impl Responder {
    match app_data.service_manager.course_service.create(&data).await {
        Ok(result) => match result.inserted_id.as_object_id() {
            Some(id) => HttpResponse::Ok().json(id.to_hex()),
            None => HttpResponse::InternalServerError().body("Failed to extract inserted_id"),
        },
        Err(e) => {
            eprintln!("Error while adding course: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add the course")
        }
    }
}

/// Route to update an existing course by its MongoDB `_id`
#[put("/courses/update/{course_id}")]
async fn update(
    app_data: web::Data<crate::AppState>,
    data: web::Json<Course>,
    course_id: web::Path<String>,
) -> impl Responder {
    let id = course_id.into_inner(); // Extract `course_id` as a String
    match app_data.service_manager.course_service.update(&data, &id).await {
        Ok(result) => {
            if result.modified_count > 0 {
                HttpResponse::Ok().json("Course updated successfully")
            } else {
                HttpResponse::NotFound().body("Course not found or no changes made")
            }
        }
        Err(e) => {
            eprintln!("Error while updating course: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update the course")
        }
    }
}

/// Route to delete a course by its MongoDB `_id`
#[delete("/courses/delete/{course_id}")]
async fn delete(
    app_data: web::Data<crate::AppState>,
    course_id: web::Path<String>,
) -> impl Responder {
    let id = course_id.into_inner(); // Extract `course_id` as a String
    match app_data.service_manager.course_service.delete(&id).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().json("Course deleted successfully")
            } else {
                HttpResponse::NotFound().body("Course not found")
            }
        }
        Err(e) => {
            eprintln!("Error while deleting course: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete the course")
        }
    }
}

/// Initialize the routes for the application
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(add);
    cfg.service(update);
    cfg.service(delete);
}

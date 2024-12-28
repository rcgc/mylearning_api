use crate::models::user_model::User;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

/// Route to get all users
#[get("/users")]
async fn get_all(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data.service_manager.user_service.get_all().await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error while getting users: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve users")
        }
    }
}

/// Route to get a users by its MongoDB `_id`
#[get("/users/{id}")]
async fn get_by_id(
    app_data: web::Data<crate::AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    let id = user_id.into_inner();
    match app_data.service_manager.user_service.get_by_id(&id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Error while getting user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve the user")
        }
    }
}

/// Route to add a new user
#[post("/users")]
async fn add(app_data: web::Data<crate::AppState>, data: web::Json<User>) -> impl Responder {
    // Clone the user data and hash the password
    let mut user = data.into_inner();

    // Hash the password before saving
    match bcrypt::hash(user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => {
            user.password = hashed_password; // Replace the plain-text password with the hash

            // Attempt to insert the user into the database
            match app_data.service_manager.user_service.create(&user).await {
                Ok(result) => match result.inserted_id.as_object_id(){
                    // Safely extract and return the MongoDB-generated `_id`
                    Some(id) => HttpResponse::Ok().json(id.to_hex()),
                    None => HttpResponse::InternalServerError().body("Failed to extract inserted_id as ObjectId"),
                },
                Err(e) => {
                    eprintln!("Error while adding user: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to add the user")
                }
            }
        }
        Err(e) => {
            eprintln!("Error while hashing password: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to hash the password")
        }
    }
}

/// Route to update an existing user by its MongoDB `_id`
#[put("/users/{id}")]
async fn update(
    app_data: web::Data<crate::AppState>,
    data: web::Json<User>,
    user_id: web::Path<String>,
) -> impl Responder {
    let id = user_id.into_inner();
    match app_data.service_manager.user_service.update(&data, &id).await {
        Ok(result) => {
            if result.modified_count > 0 {
                HttpResponse::Ok().json("User updated successfully")
            } else {
                HttpResponse::NotFound().body("User not found or no changes made")
            }
        }
        Err(e) => {
            eprintln!("Error while updating user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update the user")
        }
    }
}

#[delete("/users/{id}")]
async fn delete(
    app_data: web::Data<crate::AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    let id = user_id.into_inner();
    match app_data.service_manager.user_service.delete(&id).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().json("User deleted successfully")
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(e) => {
            eprintln!("Error while deleting user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete the user")
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
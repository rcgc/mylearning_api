use crate::models::course_search_model::CourseSearchParams;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::doc;

#[post("/courses/search")]
async fn search_courses(
    app_data: web::Data<crate::AppState>,
    body: web::Json<CourseSearchParams>,
) -> impl Responder {
    let mut filters = vec![];

    // Add filters only if they are provided
    if let Some(ref title) = body.title {
        filters.push(doc! { "title": { "$regex": title, "$options": "i" } });
    }
    if let Some(ref author) = body.author {
        filters.push(doc! { "author": { "$regex": author, "$options": "i" } });
    }
    if let Some(ref platform) = body.platform {
        filters.push(doc! { "platform": { "$regex": platform, "$options": "i" } });
    }
    if let Some(ref topics) = body.topics {
        // Ensure topics is an array of strings and match any topic
        filters.push(doc! { "topics": { "$in": topics.clone() } });
    }

    // Ensure at least one filter is provided
    if filters.is_empty() {
        return HttpResponse::BadRequest().body("At least one search parameter must be provided");
    }

    // Build the query using `$or` to combine conditions
    let filter_doc = doc! { "$or": filters };

    // Perform the search in the database
    match app_data.service_manager.course_search_service.search(filter_doc).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            eprintln!("Error while searching courses: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to search courses")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(search_courses);
}
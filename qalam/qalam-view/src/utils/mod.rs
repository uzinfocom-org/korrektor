use actix_web::{get, HttpResponse};

pub mod duplicate;
pub mod frequency;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Available tools: duplicate, frequency")
}

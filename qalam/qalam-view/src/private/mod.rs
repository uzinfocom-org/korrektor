use actix_web::{get, HttpResponse};

pub mod transliterate;
pub mod correct;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Available tools: correct, transliterate")
}
use actix_web::{get, HttpResponse};

pub mod correct;
pub mod transliterate;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Available tools: correct, transliterate")
}

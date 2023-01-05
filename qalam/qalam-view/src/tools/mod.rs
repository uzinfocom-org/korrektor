use actix_web::{get, HttpResponse};

pub mod alphabetic;
pub mod number;
pub mod tokenize;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Available tools: number, tokenize, alphabetic")
}

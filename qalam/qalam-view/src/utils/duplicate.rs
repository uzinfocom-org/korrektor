use actix_web::{get, web, HttpResponse};

#[get("/duplicate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Duplicates module")
}

#[get("/duplicate/{content}")]
pub async fn content(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner()))
}

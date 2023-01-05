use actix_web::{get, web, HttpResponse};

#[get("/number/{content}")]
pub async fn main(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner()))
}

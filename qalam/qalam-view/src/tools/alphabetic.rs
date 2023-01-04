use actix_web::{get, web, HttpResponse};

#[get("/show/{id}")]
pub async fn sort(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

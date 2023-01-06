use actix_web::{get, web, HttpResponse};
use korrektor::utils::frequency;
use serde_json::json;

#[get("/frequency")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Frequency module")
}

#[get("/frequency/{content}")]
pub async fn content(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();
    let process = frequency::count(content.clone().as_str());

    HttpResponse::Ok().json(json!({
        "message": "utils/frequency",
        "query": content,
        "content": process
    }))
}

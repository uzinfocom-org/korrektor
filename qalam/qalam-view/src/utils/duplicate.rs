use actix_web::{get, web, HttpResponse};
use korrektor::utils::duplicates;
use serde_json::json;

#[get("/duplicate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Duplicates module")
}

#[get("/duplicate/{content}")]
pub async fn content(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();

    let process = duplicates::remove(content.clone().as_str());

    HttpResponse::Ok().json(json!({
        "message": "utils/duplicate",
        "query": content,
        "content": process
    }))
}

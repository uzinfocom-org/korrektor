use actix_web::{get, web, HttpResponse};
use korrektor::utils::frequency;
use serde_json::json;

#[get("/freq/{content}")]
pub async fn count(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();

    let process = frequency::count(content.clone().as_str());

    HttpResponse::Ok().json(json!({
        "module": "utils/frequency",
        "query": content,
        "content": process
    }))
}

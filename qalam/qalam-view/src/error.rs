use actix_web::HttpResponse;
use serde_json::json;

pub async fn index() -> HttpResponse {
    HttpResponse::NotFound().json(json!({"message": "Not found"}))
}

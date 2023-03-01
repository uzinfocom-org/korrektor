use actix_web::{get, HttpResponse};
use serde_json::json;

pub mod duplicate;
pub mod frequency;

#[get("")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "utils",
        "endpoints": [
            {
                "url": "/duplicate",
                "docs": "https://docs.korrektor.uz/duplicate"
            },
            {
                "url": "/frequency",
                "description": "https://docs.korrektor.uz/frequency"
            }
        ]
    }))
}

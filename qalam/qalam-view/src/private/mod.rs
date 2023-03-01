use actix_web::{get, HttpResponse};
use serde_json::json;

pub mod correct;
pub mod transliterate;

#[get("")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "private",
        "endpoints": [
            {
                "url": "/correct",
                "docs": "https://docs.korrektor.uz/correct"
            },
            {
                "url": "/transliterate",
                "description": "https://docs.korrektor.uz/transliterate"
            },
        ]
    }))
}

use serde_json::json;
use actix_web::{get, HttpResponse};

pub mod alphabetic;
pub mod number;
pub mod tokenize;

#[get("")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "tools",
        "endpoints": [
            {
                "url": "/number",
                "docs": "https://docs.korrektor.uz/number"
            },
            {
                "url": "/tokenize",
                "description": "https://docs.korrektor.uz/tokenize"
            },
            {
                "url": "/alphabetic",
                "description": "https://docs.korrektor.uz/alphabetic"
            }
        ]
    }))
    
}

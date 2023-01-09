use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::alphabetic;
use serde_json::json;

#[get("/alphabetic")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Alphabetic ordering module")
}

#[get("/alphabetic/{content}")]
pub async fn content(path: web::Path<String>, auth: BearerAuth) -> HttpResponse {
    let content = path.into_inner();
    let process = alphabetic::sort(content.as_str());

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/alphabetic",
            "query": content,
            "content": process
        })),
        auth,
    )
}

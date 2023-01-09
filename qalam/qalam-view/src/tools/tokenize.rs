use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::tokenize;
use serde_json::json;

#[get("/tokenize")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Tokenize module")
}

#[get("/tokenize/{content}")]
pub async fn content(path: web::Path<String>, auth: BearerAuth) -> HttpResponse {
    let content = path.into_inner();
    let process = tokenize::split_word(content.as_str());

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/tokenize",
            "query": content,
            "content": process
        })),
        auth,
    )
}

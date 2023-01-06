use actix_web::{get, web, HttpResponse};
use korrektor::uzbek::tokenize;
use serde_json::json;

#[get("/tokenize")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Tokenize module")
}

#[get("/tokenize/{content}")]
pub async fn content(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();
    let process = tokenize::split_word(content.as_str());

    HttpResponse::Ok().json(json!({
        "message": "tools/tokenize",
        "query": content,
        "content": process
    }))
}

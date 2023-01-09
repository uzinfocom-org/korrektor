use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::number;
use serde_json::json;

#[get("/number")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Number module")
}

#[get("/number/{content}")]
pub async fn content(path: web::Path<i64>, auth: BearerAuth) -> HttpResponse {
    let content = path.into_inner();
    let process = number::integer_to_word(content);

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/number",
            "query": content,
            "content": process
        })),
        auth,
    )
}

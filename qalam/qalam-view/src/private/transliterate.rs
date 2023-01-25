use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor_rs_private;
use serde_json::json;


#[get("/transliterate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Transliteration module")
}

#[get("/transliterate/{lang}/{content}")]
pub async fn content(path: web::Path<(String, String)>, auth: BearerAuth) -> HttpResponse {
    let (language, content) = path.into_inner();
    let process = korrektor_rs_private::transliterator::to(content.clone(), &language);

    middleware(
        HttpResponse::Ok().json(json!({
        "message": "private/transliterate",
        "query": content,
        "content": process
    })),
        auth,
    )
}
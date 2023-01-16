use actix_web::{get, web, HttpResponse};
use serde_json::json;
use korrektor_rs_private;

#[get("/transliterate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Transliteration module")
}

#[get("/transliterate/{lang}/{content}")]
pub async fn content(path: web::Path<(String, String)>) -> HttpResponse {
    let (language, content) = path.into_inner();
    let process = korrektor_rs_private::transliterator::to(content.clone(), &language);

    HttpResponse::Ok().json(json!({
        "message": "private/transliterate",
        "query": content,
        "content": process
    }))
}
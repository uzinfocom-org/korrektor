use actix_web::{get, web, HttpResponse};
use serde_json::json;
use korrektor_rs_private;

#[get("/correct")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Correction module")
}

#[get("/correct/content/{lang}/{content}")]
pub async fn content(path: web::Path<(String, String)>) -> HttpResponse {
    let (language, content) = path.into_inner();
    let process = korrektor_rs_private::corrector::get_correction_suggestions(&content, &language);

    HttpResponse::Ok().json(json!({
        "message": "private/correct",
        "query": content,
        "content": process
    }))
}

#[get("/correct/modifiers/{content}")]
pub async fn modifiers(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();
    let process = korrektor_rs_private::corrector::remove_modifiers(&content);

    HttpResponse::Ok().json(json!({
        "message": "private/correct",
        "query": content,
        "content": process
    }))
}

#[get("/correct/syntax/{content}")]
pub async fn syntax(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();
    let process = korrektor_rs_private::corrector::correct(&content);

    HttpResponse::Ok().json(json!({
        "message": "private/correct",
        "query": content,
        "content": process
    }))
}
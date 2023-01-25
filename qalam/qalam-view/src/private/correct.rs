use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor_rs_private;
use serde_json::json;

#[get("/correct")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Correction module")
}

#[get("/correct/content/{lang}/{content}")]
pub async fn content(path: web::Path<(String, String)>, auth: BearerAuth) -> HttpResponse {
    let (language, content) = path.into_inner();
    let process = korrektor_rs_private::corrector::get_correction_suggestions(&content, &language);

    middleware(
        HttpResponse::Ok().json(json!({
        "message": "private/correct/content",
        "query": content,
        "content": process
    })),
        auth,
    )
}

#[get("/correct/modifiers/{text_content}")]
pub async fn modifiers(path: web::Path<String>, auth: BearerAuth) -> HttpResponse {
    let text_content = path.into_inner();
    let process = korrektor_rs_private::corrector::remove_modifiers(&text_content);

    middleware(
        HttpResponse::Ok().json(json!({
        "message": "private/correct/modifiers",
        "query": text_content,
        "content": process
    })),
        auth,
    )
}

#[get("/correct/syntax/{text_content}")]
pub async fn syntax(path: web::Path<String>, auth: BearerAuth) -> HttpResponse {
    let text_content = path.into_inner();
    let process = korrektor_rs_private::corrector::correct(&text_content);

    middleware(
        HttpResponse::Ok().json(json!({
        "message": "private/correct/syntax",
        "query": text_content,
        "content": process
    })),
        auth,
    )
}
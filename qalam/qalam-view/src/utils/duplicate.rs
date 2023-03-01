use actix_web::{get, post, web, HttpResponse};
use korrektor::utils::duplicates;
use serde_json::json;
use crate::request::Request;

#[get("/duplicate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/duplicate",
        "docs": "https://docs.korrektor.uz/duplicate"
    }))
}

#[post("/duplicate")]
pub async fn content(body: web::Json<Request>) -> HttpResponse {
    let content= body.into_inner().content;

    let process = duplicates::remove(&content);

    HttpResponse::Ok().json(json!({
        "message": "utils/duplicate",
        "query": content,
        "content": process
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn content_test() {
        let text_content = "hello sam hello";
        let process = duplicates::remove(text_content);

        let response = json!({
            "message": "utils/duplicate",
            "query": text_content,
            "content": process
        });

        let static_json =
            "{\"content\":\"hello sam\",\"message\":\"utils/duplicate\",\"query\":\"hello sam hello\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

use actix_web::{get, web, HttpResponse};
use korrektor::utils::duplicates;
use serde_json::json;

#[get("/duplicate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Duplicates module")
}

#[get("/duplicate/{content}")]
pub async fn content(path: web::Path<String>) -> HttpResponse {
    let content = path.into_inner();

    let process = duplicates::remove(content.as_str());

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

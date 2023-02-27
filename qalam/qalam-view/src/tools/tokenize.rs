use crate::auth::middleware;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::tokenize;
use serde_json::json;

#[get("/tokenize")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/tokenize",
        "docs": "https://docs.korrektor.uz/tokenize"
    }))
}

#[post("/tokenize")]
pub async fn content(path: web::Bytes, auth: BearerAuth) -> HttpResponse {
    let content = match String::from_utf8(path.to_vec()) {
        Ok(string) => string,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "message": "tools/tokenize",
                "content": "Invalid input in body: should be text with valid characters."}));
        }
    };

    let process = tokenize::split_text(content.as_str());

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/tokenize",
            "query": content,
            "content": process
        })),
        auth,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn content_test() {
        let text_content = "singil chiroyli чиройли";
        let process = tokenize::split_text(text_content);

        let response = json!({
            "message": "tools/tokenize",
            "query": text_content,
            "content": process
        });

        let static_json =
            "{\"content\":\"si-ngil chi-roy-li чи-рой-ли\",\"message\":\"tools/tokenize\",\"query\":\"singil chiroyli чиройли\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

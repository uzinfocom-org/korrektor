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

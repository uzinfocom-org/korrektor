use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::alphabetic;
use serde_json::json;

#[get("/alphabetic")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/alphabetic",
        "docs": "https://docs.korrektor.uz/alphabetic"
    }))
}

#[post("/alphabetic")]
pub async fn content(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let content = body.into_inner().content;

    let process = alphabetic::sort(content.as_str());

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/alphabetic",
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
        let text_content = "G‘ozal estafeta chilonzor o'zbek chiroyli";
        let process = alphabetic::sort(text_content);

        let response = json!({
            "message": "tools/alphabetic",
            "query": text_content,
            "content": process
        });

        let static_json =
            "{\"content\":\"estafeta o‘zbek chilonzor chiroyli G‘ozal\",\"message\":\"tools/alphabetic\",\"query\":\"G‘ozal estafeta chilonzor o'zbek chiroyli\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

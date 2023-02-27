use crate::auth::middleware;
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
pub async fn content(path: web::Bytes, auth: BearerAuth) -> HttpResponse {
    let content = match String::from_utf8(path.to_vec()) {
        Ok(string) => string,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "message": "tools/alphabetic",
                "content": "Invalid input in body: should be text with valid characters."}));
        }
    };

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

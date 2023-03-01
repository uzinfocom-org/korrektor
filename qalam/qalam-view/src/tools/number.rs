use crate::auth::middleware;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::number;
use serde_json::json;
use crate::request::Request;

#[get("/number")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/number",
        "docs": "https://docs.korrektor.uz/number"
    }))
}

#[post("/number")]
pub async fn content(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let content_string= body.into_inner().content;

    let content: i64 = match content_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "message": "tools/number",
                "content": "Invalid input in body: can not convert input into integer."}));
        }
    };

    let process = number::integer_to_word(content);

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/number",
            "query": content_string,
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
        let text_content = 12;
        let process = number::integer_to_word(text_content);

        let response = json!({
            "message": "tools/number",
            "query": text_content,
            "content": process
        });

        let static_json = "{\"content\":\"o‘n ikki\",\"message\":\"tools/number\",\"query\":12}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

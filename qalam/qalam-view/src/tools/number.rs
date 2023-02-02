use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::number;
use serde_json::json;

#[get("/number")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Number module")
}

#[get("/number/{content}")]
pub async fn content(path: web::Path<i64>, auth: BearerAuth) -> HttpResponse {
    let content = path.into_inner();
    let process = number::integer_to_word(content);

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "tools/number",
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
        let text_content = 12;
        let process = number::integer_to_word(text_content);

        let response = json!({
        "message": "tools/number",
        "query": text_content,
        "content": process
    });


        let static_json =
            "{\"content\":\"o‘n ikki\",\"message\":\"tools/number\",\"query\":12}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

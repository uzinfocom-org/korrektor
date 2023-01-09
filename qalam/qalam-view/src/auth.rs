use actix_web::HttpResponse;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

pub fn middleware(req: HttpResponse, token: BearerAuth) -> HttpResponse {
    if !token.token().is_empty() {
        println!("TOKEN: {}", token.token());
        req
    } else {
        HttpResponse::Unauthorized().body(
            serde_json::to_string(&json!({
                "message": "Bruh..."
            }))
            .unwrap(),
        )
    }
}

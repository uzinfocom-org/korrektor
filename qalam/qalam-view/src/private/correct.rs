use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor_rs_private;
use serde_json::json;

#[get("/correct")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/correct",
        "docs": "https://docs.korrektor.uz/correct"
    }))
}

#[post("/correct/content/{lang}")]
pub async fn content(
    path: web::Path<String>,
    body: web::Json<Request>,
    auth: BearerAuth,
) -> HttpResponse {
    let language = path.into_inner();

    let content = body.into_inner().content;

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

#[post("/correct/modifiers")]
pub async fn modifiers(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let text_content = body.into_inner().content;

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

#[post("/correct/syntax")]
pub async fn syntax(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let text_content = body.into_inner().content;

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

#[cfg(test)]
mod tests {
    use super::*;
    use korrektor_rs_private::corrector;

    #[actix_web::test]
    async fn content_lat_test() {
        let text_content_lat = "chroyli";
        let process_lat = corrector::get_correction_suggestions(text_content_lat, "lat");

        let response = json!({
            "message": "private/correct/content",
            "query": text_content_lat,
            "content": process_lat
        });

        let errors_lat = json!({
            "misspelled": "chroyli",
            "position": 0,
            "suggestions": [
                "choyli".to_string(),
                "chiroyli".to_string(),
                "chorpoyli".to_string(),
                "choroynali".to_string(),
                "choykorli".to_string(),
                "chiroyi".to_string(),
                "zichroqli".to_string()]
        });

        let static_json = "{\"content\":[".to_string()
            + &serde_json::to_string(&errors_lat).unwrap()
            + "],\"message\":\"private/correct/content\",\"query\":\"chroyli\"}";

        assert_eq!(&serde_json::to_string(&response).unwrap(), &static_json);
    }

    #[actix_web::test]
    async fn content_cyr_test() {
        let text_content_cyr = "чройли";
        let process_cyr = corrector::get_correction_suggestions(text_content_cyr, "cyr");

        let response = json!({
            "message": "private/correct/content",
            "query": text_content_cyr,
            "content": process_cyr
        });

        let errors_cyr = json!({
            "misspelled": "чройли",
            "position": 0,
            "suggestions": [
                "чойли".to_string(),
                "чиройли".to_string(),
                "чорпойли".to_string(),
                "ойлили".to_string(),
                "ойликчи".to_string(),
                "чоройнали".to_string(),
                "бройлерли".to_string()]
        });

        let static_json = "{\"content\":[".to_string()
            + &serde_json::to_string(&errors_cyr).unwrap()
            + "],\"message\":\"private/correct/content\",\"query\":\"чройли\"}";

        assert_eq!(&serde_json::to_string(&response).unwrap(), &static_json);
    }

    #[actix_web::test]
    async fn modifiers_test() {
        let text_content = "stul- stul-ku";
        let process = corrector::remove_modifiers(text_content);

        let response = json!({
            "message": "private/correct/modifiers",
            "query": text_content,
            "content": process
        });

        let static_json =
            "{\"content\":\"stul  stul\",\"message\":\"private/correct/modifiers\",\"query\":\"stul- stul-ku\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }

    #[actix_web::test]
    async fn syntax_test() {
        let text_content = "2022-йил 12 yanvar go'zal";
        let process = corrector::correct(text_content);

        let response = json!({
            "message": "private/correct/syntax",
            "query": text_content,
            "content": process
        });

        let static_json =
            "{\"content\":\"2022 йил 12-yanvar go‘zal\",\"message\":\"private/correct/syntax\",\"query\":\"2022-йил 12 yanvar go'zal\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}

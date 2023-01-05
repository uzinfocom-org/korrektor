use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, Error, HttpResponse, Responder};
use serde_json::json;

pub mod error;
pub mod stats;
pub mod tools;
pub mod utils;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!(
            {
                "message": "welcome to korrektor's backend, check the api health /status"
            }
    ))
}

#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = "./favicon.ico".parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

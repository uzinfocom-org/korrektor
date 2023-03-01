use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, Error, HttpResponse, Responder};

pub mod auth;
pub mod error;
pub mod private;
pub mod stats;
pub mod tools;
pub mod utils;

#[get("/")]
pub async fn index() -> impl Responder {
    // Send sample json response
    // HttpResponse::Ok().json(json!(
    //         {
    //             "message": "welcome to korrektor's backend, check the api health /status"
    //         }
    // ))

    // Respond with a redirect
    HttpResponse::Found()
        .append_header(("Location", "https://docs.korrektor.uz"))
        .finish()
}

#[get("/favicon.ico")]
pub async fn favicon() -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = "./favicon.ico".parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

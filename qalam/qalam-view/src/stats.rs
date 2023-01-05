use actix_web::{get, web, HttpResponse, Responder};
use peak_alloc::PeakAlloc;
use serde_json::json;

#[derive(Clone)]
pub struct Status {
    pub peak: PeakAlloc,
    pub version: &'static str,
}

#[get("/status")]
async fn index(status: web::Data<Status>) -> impl Responder {
    let data = status.as_ref();

    HttpResponse::Ok().json(json!({
         "package": "korrektor-http",
         "version": format!("{}", data.version),
         "message": "Everything is operational! (Usage in MBs)",
         "usage": {
             "current": data.peak.current_usage_as_mb(),
             "peak": data.peak.peak_usage_as_mb()
         }
    }))
}

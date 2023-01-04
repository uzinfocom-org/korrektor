use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use qalam_view::{tools, utils};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from the backend!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3001;
    let address = "127.0.0.1";
    println!("ready - started server on {address}:{port}, url: http://{address}:{port}");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(tools::scope())
            .service(utils::scope())
    })
    .bind((address, port))?
    .run()
    .await
}

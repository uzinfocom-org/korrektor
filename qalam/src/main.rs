use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body( "Hello from the backend!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3001;
    let address = "127.0.0.1";
    println!("ready - started server on {address}:{port}, url: http://localhost:{port}", address=address, port=port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind((address, port))?
        .run()
        .await
}

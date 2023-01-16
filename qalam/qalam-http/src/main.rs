use actix_web::middleware::TrailingSlash;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::{self};
use peak_alloc::PeakAlloc;
use qalam_view::stats::Status;
use qalam_view::{error, favicon, index, stats, tools, utils};

pub mod init;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initializing logger & envs
    init::initialize().await;

    // Creating status checker
    let status: Status = Status {
        peak: PEAK_ALLOC,
        version: VERSION,
    };

    // Logging the outlet of the server
    let configs: (String, u16) = init::target();
    println!(
        "ready - started server on {address}:{port}, url: https://{address}:{port}",
        address = configs.0,
        port = configs.1
    );

    // Start the server on X amount of core(s)
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(status.clone()))
            .app_data(bearer::Config::default().realm("Restricted area: Dungeon Masters only"))
            .service(index)
            .service(favicon)
            .service(stats::index)
            .service(
                web::scope("/tools")
                    // Main content
                    .service(tools::index)
                    // Number to word
                    .service(tools::number::main)
                    .service(tools::number::content)
                    // Tokenizer
                    .service(tools::tokenize::main)
                    .service(tools::tokenize::content)
                    // Alphabetical ordering
                    .service(tools::alphabetic::main)
                    .service(tools::alphabetic::content),
            )
            .service(
                web::scope("/utils")
                    // Main content
                    .service(utils::index)
                    // Duplicates
                    .service(utils::duplicate::main)
                    .service(utils::duplicate::content)
                    // Frequency
                    .service(utils::frequency::main)
                    .service(utils::frequency::content),
            )
            .default_service(web::route().to(error::index))
    })
    .bind((configs.0, configs.1))?
    .run()
    .await
}
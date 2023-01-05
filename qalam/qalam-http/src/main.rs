use actix_web::{middleware, web, App, HttpServer};
use peak_alloc::PeakAlloc;
use qalam_view::stats::Status;
use qalam_view::{error, index, stats, tools, utils};

mod init;

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
        "ready - started server on {address}:{port}, url: http://{address}:{port}",
        address = configs.0,
        port = configs.1
    );

    // Start the server on X amount of core(s)
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .app_data(web::Data::new(status.clone()))
            .service(index)
            .service(stats::index)
            .service(
                web::scope("/tools")
                    .service(tools::index)
                    .service(tools::number::main)
                    .service(tools::tokenize::main)
                    .service(tools::alphabetic::main),
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

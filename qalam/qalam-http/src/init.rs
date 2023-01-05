pub async fn initialize() {
    // Initialize dotenv configurations
    dotenv::dotenv().ok();

    let is_dev = match std::env::var("ISDEV") {
        Ok(host) => host,
        Err(_) => "no".to_owned(),
    };

    if is_dev == "yes" {
        println!("Server has been started in Debug mode!");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    } else {
        println!("Server has been started in Production mode!");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    }
}

pub fn target() -> (String, u16) {
    // Define the target of host
    (
        std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
        std::env::var("PORT")
            .unwrap_or_else(|_| 3001.to_string())
            .parse()
            .unwrap(),
    )
}

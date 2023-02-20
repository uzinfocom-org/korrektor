pub async fn initialize() {
    if std::env::var("ISDEV").unwrap() == "yes" {
        println!("Server has been started in Debug mode!");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    } else {
        println!("Server has been started in Production mode!");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    }
}

pub fn target() -> (String, u16) {
    (
        std::env::var("HOST").unwrap(),
        std::env::var("PORT").unwrap().parse().unwrap(),
    )
}

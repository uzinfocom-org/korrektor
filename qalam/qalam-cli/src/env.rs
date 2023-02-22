use crate::statics::ENV;

pub fn env_create() {
    std::fs::write("./.env", ENV).expect("Couldn't write .env file");
    println!("Generated template .env file on your current working directory");
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn env_map(maps: Vec<(&str, &str)>) {
    // Env setter announcement
    println!();
    println!("Setting env keys as the following:");

    for map in maps {
        println!("{}: {}", map.0, map.1);
        std::env::set_var(map.0, map.1);
    }
    println!();
}

pub fn env_check() {
    match std::path::Path::new("./.env").exists() {
        true => {
            println!("Found .env file, loading configurations");
            dotenv::dotenv().ok();
        }
        false => {
            println!("Can't locate .env file, loading default settings");
            env_map(vec![
                ("ISDEV", "yes"),
                ("HOST", "127.0.0.1"),
                ("PORT", "8083"),
                ("THREADS", "1"),
            ]);
        }
    }
}

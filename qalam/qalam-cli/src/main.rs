mod env;
mod skin;
mod data;

use std::io::Write;
use crate::env::env_create;
use clap::{Parser, Subcommand};

const FIGLET: &str = include_str!("./figlet");
const TUTORIAL: &str = include_str!("./tutorial.md");

/// Qalam rest api service made for Korrektor API.
#[derive(Debug, Parser)]
#[clap(name = "qalam")]
#[clap(about = "Qalam rest api service made for Korrektor API.", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate .env template file with required keys
    Env,

    /// Show tutorial how to setup to get binary working!
    Tutorial,

    /// Starting the rest server
    Server,
    
    /// Install and unzip latest hunspell dictionary
    Data,
}

#[actix_web::main]
async fn main() {
    println!("{}", FIGLET);

    let args = Cli::parse();

    match args.command {
        Commands::Server => {
            // Initialize dotenv configurations
            env::env_check();

            // Start the server
            qalam_http::server()
                .await
                .expect("Couldn't start the server");
        }
        Commands::Env => {
            // Generate env file
            match std::path::Path::new("./.env").exists() {
                true => {
                    print!("Found .env file, would you like to reset existing one? [y/N] ");
                    
                    // IO stdin catch
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    let stdin = std::io::stdin();
                    stdin
                        .read_line(&mut input)
                        .expect("error occurred while prepending stdin");

                    // Remove trailing new line
                    env::trim_newline(&mut input);
                    
                    match input.as_str() {
                        "Y" | "y" => {
                            println!("Ok, existing env file will be resetted.");
                            env::env_create();
                        }
                        "N" | "n" => {
                            println!("Got it, won't touch existing configuration!")
                        }
                        _ => {
                            println!("I actually didn't understand, but I assume this as no.")
                        }
                    }
                }
                false => env_create(),
            }
        }
        Commands::Tutorial => {
            // Render markdown on terminal
            let help = skin::wrapper();
            help.print_inline(TUTORIAL);
        }
        Commands::Data => {
            data::bootstrap().await;
            
            println!("Still WIP");
        }
    }
}

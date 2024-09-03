use clap::{Arg, Command};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info, warn, LevelFilter};
use reqwest::Client;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    info!("Starting template_cli...");

    let matches = Command::new("template_cli")
        .version("1.0")
        .about("A template CLI application")
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Sets the level of logging to DEBUG")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("info")
                .long("info")
                .help("Sets the level of logging to INFO")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("warn")
                .long("warn")
                .help("Sets the level of logging to WARN")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("error")
                .long("error")
                .help("Sets the level of logging to ERROR")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("off")
                .long("off")
                .help("Turns off logging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let log_level = match () {
        _ if matches.get_flag("debug") => LevelFilter::Debug,
        _ if matches.get_flag("info") => LevelFilter::Info,
        _ if matches.get_flag("warn") => LevelFilter::Warn,
        _ if matches.get_flag("error") => LevelFilter::Error,
        _ if matches.get_flag("off") => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();
    debug!("Logging level set to {:?}", log_level);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let env_path = current_dir.join(".env");

    if env_path.exists() {
        dotenv().ok();
        info!("Loaded .env file from {:?}", env_path);
    } else {
        error!("No .env file found in the current directory: {:?}", current_dir);
        return;
    }

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found in .env file");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env file");

    debug!("CLIENT_ID: {}, CLIENT_SECRET: [hidden]", client_id);

    let client = Client::new();
    let response = client
        .post("https://api.datto.com/v1/saas/domains")
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let access_token: String = res.text().await.unwrap();
                info!("Access token retrieved: {}", access_token);
            } else {
                error!("Failed to retrieve access token: {}", res.status());
            }
        }
        Err(e) => {
            error!("Error connecting to Datto API: {:?}", e);
        }
    }

    info!("template_cli has finished execution.");
}

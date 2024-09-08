use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;  // For environment-based logging

mod api;
mod db;
mod services;
mod config;

use config::load_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env variables
    dotenv().ok();

    // Initialize the logger, with optional RUST_LOG configuration from the environment
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load other configurations and set up the connection pool
    load_config();
    let pool = db::connect().await.expect("Failed to connect to the database");

    log::info!("Starting HTTP server at http://127.0.0.1:8080");  // Logging startup info

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

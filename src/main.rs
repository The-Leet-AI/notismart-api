use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger::Env;
use config::load_config;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi; // This imports the OpenApi trait that provides the `openapi()` method.
use swagger::ApiDoc;  // Adjust the module import to where you have defined `ApiDoc`.

mod auth;
mod api;
mod db;
mod services;
mod config;
mod swagger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = load_config();
    let pool = db::connect(&config.database_url).await.expect("Failed to connect to the database");
    
    let openapi = swagger::ApiDoc::openapi();  // Generate OpenAPI specification from the new file

    log::info!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
            .configure(api::init_routes)
            .route("/", web::get().to(|| async { "Hello, API!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

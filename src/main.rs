use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger::Env;
use config::load_config;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
mod auth;
mod api;
mod db;
mod services;
mod config;


use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(api::user::create_user, api::notification::create_notification),
    components(
        schemas(api::user::CreateUser, api::notification::CreateNotification, api::notification::NotificationResponse, db::models::Notification)
    ),
    tags(
        (name = "User API", description = "User-related endpoints"),
        (name = "Notification API", description = "Notification-related endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "BearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    load_config();
    let pool = db::connect().await.expect("Failed to connect to the database");

    let openapi = ApiDoc::openapi(); // Generate OpenAPI specification

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

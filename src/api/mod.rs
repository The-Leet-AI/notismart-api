pub mod notification;
pub mod user;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(notification::init_routes) // Add notification routes
            .configure(user::init_routes)         // Add user routes
    );
}

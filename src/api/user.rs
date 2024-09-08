use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Define user-related routes here, for example:
    cfg.route("/users", web::get().to(get_users));
}

// Example function for the sake of completeness
async fn get_users() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json("Get users route")
}

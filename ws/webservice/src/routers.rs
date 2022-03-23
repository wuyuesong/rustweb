use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/courses")   //用于定义作用域，换句话说就是一套资源的根路径
    .route("/",web::post().to(new_course)));
}
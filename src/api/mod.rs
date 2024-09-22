use actix_web::web;
use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/").route(web::get().to(handlers::hello::hello_world)),
    )
    .service(
        web::resource("/greet").route(web::get().to(handlers::hello::greet)),
    )
    .service(
        web::resource("/api/file/node/status").route(web::get().to(handlers::node::get_status)),
    );
}

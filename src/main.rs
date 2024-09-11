use actix_web::{web, App, HttpServer};
mod api;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(api::routes) // Load routes from api module
    })
    .bind("127.0.0.1:3008")?
    .run()
    .await
}

use actix_web::{web, App, HttpServer};
use std::io::Error;

// Placeholder for your routes
async fn some_route() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            // Define your routes here, for example:
            .route("/", web::get().to(some_route)) // Bind routes to specific functions
    })
    .bind("127.0.0.1:3008")?  // Bind to port 3008
    .run()
    .await
}

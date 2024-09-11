use actix_web::{HttpResponse, Responder};

pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Greetings from the server!")
}
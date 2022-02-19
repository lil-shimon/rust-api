use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn show() -> impl Responder {
  HttpResponse::Ok().json("test")
}
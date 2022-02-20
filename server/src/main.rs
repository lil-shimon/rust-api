use actix_web::{App, HttpServer};
use rust_api::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(routes::routes)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
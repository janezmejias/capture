use crate::infrastructure::web::handler::user_handler;
use actix_web::{App, HttpServer};

mod application;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_handler))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

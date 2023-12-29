use crate::infrastructure::web::handler::{create, user_by_id};
use actix_web::{App, HttpServer};

mod application;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_by_id).service(create))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

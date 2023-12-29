use crate::application::use_cases::find_user;
use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Path;
use crate::infrastructure::web::dto::user_request::UserRequest;

#[get("/v1/user/{id}")]
pub async fn user_handler(params: Path<UserRequest>) -> impl Responder {
    let user_id = params.id;
    match find_user(user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("error looking-for user"),
    }
}

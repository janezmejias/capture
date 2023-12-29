use crate::application::use_cases::find_user;
use crate::infrastructure::web::dto::user_request::UserRequest;
use actix_web::web::{Json, Path};
use actix_web::{get, post, HttpResponse, Responder};
use validator::Validate;
use crate::domain::entities::User;

#[get("/v1/user/{id}")]
pub async fn user_by_id(params: Path<UserRequest>) -> impl Responder {
    let user_id = params.id;
    match find_user(user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("error looking-for user"),
    }
}

#[post("/v1/user")]
pub async fn create(body: Json<User>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().json(body),
        Err(_) => HttpResponse::InternalServerError().body("error looking-for user"),
    }
}

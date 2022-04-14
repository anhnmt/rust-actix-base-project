use actix_web::{get, HttpResponse, Responder};

#[get("/users")]
pub async fn users() -> impl Responder {
    HttpResponse::Ok().body("User!")
}

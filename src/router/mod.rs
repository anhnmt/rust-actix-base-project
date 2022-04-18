use actix_web::{get, post, web, HttpResponse, Responder};
use tracing_actix_web::RequestId;

use user::users;

pub mod user;

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(echo);
    cfg.service(manual_hello);
    cfg.service(users);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello(request_id: RequestId) -> impl Responder {
    HttpResponse::Ok().body(request_id.to_string())
}

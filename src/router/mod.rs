use actix_web::{get, HttpResponse, post, Responder, web};
use tracing_actix_web::RequestId;

use user::users;
use crate::models::Status;

pub mod user;

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(echo);
    cfg.service(manual_hello);
    cfg.service(users);
}

/// 404 Not Found
pub async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Hello world!".to_string(),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello(request_id: RequestId) -> impl Responder {
    HttpResponse::Ok().body(request_id.to_string())
}

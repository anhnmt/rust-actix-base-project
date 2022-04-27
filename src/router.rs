use actix_web::{get, HttpResponse, Responder, web};
use tracing_actix_web::RequestId;

use crate::{service};
use crate::model::Status;

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(manual_hello);

    // user routes
    cfg.service(service::user::find_all_users)
        .service(service::user::create_user);
}

/// 404 Not Found
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(Status {
        status: "Error 404".to_string(),
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Hello world!".to_string(),
    })
}

#[get("/hey")]
async fn manual_hello(request_id: RequestId) -> impl Responder {
    HttpResponse::Ok().body(request_id.to_string())
}

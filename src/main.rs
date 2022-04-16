use std::{env, io};

use actix_cors::Cors;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;

use crate::logger::new_logger;

// External modules reference
mod logger;
mod repo;
mod router;

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    new_logger();
    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("starting HTTP server at http://localhost:{}", app_port);

    HttpServer::new(move || {
        let logger = Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#);
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_header("*")
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(NormalizePath::new(Default::default()))
            .wrap(logger)
            .wrap(cors)
            .configure(router::init)
            .default_service(web::route().to(not_found))
    })
    .bind(&format!("0.0.0.0:{}", app_port))
    .expect(&format!("Can not bind to http://localhost:{}", app_port))
    .run()
    .await
}

/// 404 Not Found
async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}

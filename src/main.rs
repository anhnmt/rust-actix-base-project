use std::{env, io};

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::{Logger, NormalizePath};
use log::info;

use crate::logger::new_logger;

// External modules reference
mod router;
mod repo;
mod logger;
mod model;

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    new_logger();
    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("starting HTTP server at http://localhost:{}", app_port);

    HttpServer::new(|| {
        let logger = Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#);

        App::new()
            .wrap(NormalizePath::new(Default::default()))
            .wrap(logger)
            // .app_data(AppState::new())
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
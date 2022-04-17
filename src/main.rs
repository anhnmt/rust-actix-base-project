extern crate dotenvy;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::{Logger, NormalizePath};
use dotenvy::dotenv;
use log::info;
use tracing::instrument::WithSubscriber;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::time;
use tracing_subscriber::util::SubscriberInitExt;

// External modules reference
mod router;

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    // Initialize tracing
    let mut subscriber = tracing_subscriber::fmt()
        .with_timer(time::ChronoUtc::rfc3339());

    // App mode
    // let app_dev = env::var("APP_DEV").is_ok();
    // if !app_dev {
    //     subscriber = subscriber.json();
    // }

    subscriber.finish().init();

    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("Starting HTTP server at http://localhost:{}", app_port);

    HttpServer::new(move || {
        let logger = Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#);
        let tracing = TracingLogger::default();
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_header("*")
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(NormalizePath::new(Default::default()))
            .wrap(tracing)
            .wrap(logger)
            .wrap(cors)
            .configure(router::init)
            .default_service(web::route().to(not_found))
    })
        .bind(&format!("0.0.0.0:{}", app_port))?
        .run()
        .await
}

/// 404 Not Found
async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}

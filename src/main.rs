extern crate dotenvy;

use std::io;

use actix_web::{App, HttpServer, middleware};
use dotenvy::dotenv;
use log::info;

// External modules reference
mod router;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(router::init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

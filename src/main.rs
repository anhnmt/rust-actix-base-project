extern crate dotenvy;

use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::{Logger, NormalizePath};
use dotenvy::dotenv;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use log::info;

// External modules reference
mod router;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    fast_log::init(Config::new()
        .console()
        .file_split("logs/",
                    LogSize::MB(1),
                    RollingType::All,
                    LogPacker {})
    ).unwrap();

    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let logger = Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#);

        App::new()
            .wrap(NormalizePath::new(Default::default()))
            .wrap(logger)
            .configure(router::init)
            .default_service(web::route().to(not_found))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}
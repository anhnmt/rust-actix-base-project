extern crate dotenvy;

use std::io;

use actix_web::{App, HttpServer, middleware};
use dotenvy::dotenv;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use log::info;
use middleware::Logger;

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
        App::new()
            .wrap(Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T %D"#))
            .configure(router::init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

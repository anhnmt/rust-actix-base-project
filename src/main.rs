extern crate dotenvy;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compat, Compress, Logger, NormalizePath};
use actix_web::web::Data;
use bson::doc;
use dotenvy::dotenv;
use log::info;
use mongodb::Client;
use mongodb::options::ClientOptions;
use tracing_actix_web::TracingLogger;

use crate::model::AppState;

// External modules reference
mod logger;
mod model;
mod service;
mod utils;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    logger::init();

    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("Starting HTTP server at http://localhost:{}", app_port);

    // A Client is needed to connect to MongoDB:
    let client_uri = env::var("DB_URL").expect("DB_URL env not set.");
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;
    let db_name = env::var("DB_NAME").expect("DB_NAME env not set.");
    let db = client.database(&db_name);

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    info!("Connected successfully.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(Data::new(AppState {
                db: db.clone(),
                client: client.clone(),
            }))
            .wrap(NormalizePath::new(Default::default()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(Compat::new(TracingLogger::default()))
            .wrap(cors)
            .configure(router::init)
            .default_service(web::route().to(router::not_found))
    })
        .bind(&format!("0.0.0.0:{}", app_port))?
        .run()
        .await?;

    Ok(())
}

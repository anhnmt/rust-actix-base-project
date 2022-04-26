extern crate dotenvy;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::web::Data;
use bson::doc;
use dotenvy::dotenv;
use log::info;
use mongodb::Client;
use mongodb::options::ClientOptions;
use tracing_actix_web::TracingLogger;

use crate::models::AppState;

// External modules reference
mod router;
mod logger;
mod models;

#[actix_web::main] // or #[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    logger::init();

    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("Starting HTTP server at http://localhost:{}", app_port);

    // A Client is needed to connect to MongoDB:
    let client_uri = env::var("DB_URL").expect("DB_URL env not set.");
    let options = ClientOptions::parse(&client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db_name = env::var("DB_NAME").expect("DB_NAME env not set.");
    let db = client.database(&db_name);

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await.unwrap();
    info!("Connected successfully.");

    HttpServer::new(move || {
        let tracing = TracingLogger::default();
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
            .wrap(tracing)
            .wrap(cors)
            .configure(router::init)
            .default_service(web::route().to(router::not_found))
    })
        .bind(&format!("0.0.0.0:{}", app_port))?
        .run()
        .await
}

use mongodb::{Client, Database};
use serde::Serialize;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub client: Client,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}
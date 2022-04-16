use std::env;
use std::error::Error;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};

/// Repo struct
pub struct Repo {
    pub client: Client,
    pub db_name: String,
}

impl Repo {
    /// Create a new repo
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Load the MongoDB connection string from an environment variable:
        let db_url = env::var("DB_URL").expect("You must set the DB_URL environment var!");
        let db_name = env::var("DB_NAME").expect("You must set the DB_NAME environment var!");

        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let options =
            ClientOptions::parse_with_resolver_config(&db_url, ResolverConfig::cloudflare())
                .await
                .expect("Failed to parse MongoDB connection string!");
        let client = Client::with_options(options).expect("Failed to initialize client");

        // Create a new Repo instance:
        Ok(Self { client, db_name })
    }

    /// Get a database
    fn get_database(&self) -> Database {
        self.client.database(self.db_name.as_str())
    }

    /// Get a collection
    fn get_collection<T>(&self, name: String) -> Collection<T> {
        self.get_database().collection(name.as_str())
    }
}

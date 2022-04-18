use std::env;

use tracing_subscriber::fmt::time;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    // App mode
    match env::var("APP_DEV") {
        Ok(s) => {
            if s.eq_ignore_ascii_case("true") {
                return dev_logger();
            }

            return prod_logger();
        }
        _ => prod_logger(),
    }
}

fn dev_logger() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_timer(time::ChronoUtc::rfc3339())
        .finish()
        .init();
}

fn prod_logger() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_timer(time::ChronoUtc::rfc3339())
        .json()
        .finish()
        .init();
}
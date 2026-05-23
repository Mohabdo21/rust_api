#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(unused_must_use)]

use rust_api::application::error::AppError;
use rust_api::{app, config::AppConfig};
use tracing_subscriber::{EnvFilter, fmt};

/// The main entry point of the application.
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(env_filter).init();

    app::run(AppConfig::from_env()).await
}

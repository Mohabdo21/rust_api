mod api;
mod application;
mod config;
mod domain;
mod infrastructure;

use std::sync::Arc;

use application::{api_key_service::ApiKeyService, user_service::UserService};
use config::AppConfig;
use infrastructure::persistence::{
    database::connect_and_migrate,
    repositories::{
        api_key_repository::SeaOrmApiKeyRepository, user_repository::SeaOrmUserRepository,
    },
};
use sea_orm::DbErr;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(env_filter).init();

    let app_config = AppConfig::from_env();

    let db = connect_and_migrate(&app_config.database_url).await?;

    let user_repo = Arc::new(SeaOrmUserRepository::new(db.clone()));
    let api_key_repo = Arc::new(SeaOrmApiKeyRepository::new(db.clone()));

    let state = api::handlers::AppState {
        user_service: UserService::new(user_repo.clone()),
        api_key_service: ApiKeyService::new(api_key_repo, user_repo),
    };

    let app = api::routes::create_router(state);

    let bind_addr = format!("{}:{}", app_config.host, app_config.port);
    info!(address = %bind_addr, "server starting");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind tcp listener");
    info!(address = %bind_addr, "server listening");
    axum::serve(listener, app)
        .await
        .expect("failed to start server");

    Ok(())
}

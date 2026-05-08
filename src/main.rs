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

#[tokio::main]
async fn main() -> Result<(), DbErr> {
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
    println!("listening on {bind_addr}");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind tcp listener");
    axum::serve(listener, app)
        .await
        .expect("failed to start server");

    Ok(())
}

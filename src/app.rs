use std::sync::Arc;

use tracing::info;

use crate::{
    api,
    application::{api_key_service::ApiKeyService, error::AppError, user_service::UserService},
    config::AppConfig,
    infrastructure::persistence::{
        database::connect_and_migrate,
        repositories::{
            api_key_repository::DieselApiKeyRepository, user_repository::DieselUserRepository,
        },
    },
};

pub async fn build_app(database_url: &str) -> Result<axum::Router, AppError> {
    let db = connect_and_migrate(database_url).await?;

    let user_repo = Arc::new(DieselUserRepository::new(db.clone()));
    let api_key_repo = Arc::new(DieselApiKeyRepository::new(db.clone()));

    let state = api::handlers::AppState {
        user_service: UserService::new(user_repo.clone()),
        api_key_service: ApiKeyService::new(api_key_repo, user_repo),
    };

    Ok(api::routes::create_router(state))
}

pub async fn run(app_config: AppConfig) -> Result<(), AppError> {
    let app = build_app(&app_config.database_url).await?;

    let bind_addr = format!("{}:{}", app_config.host, app_config.port);
    info!(address = %bind_addr, "server starting");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!(address = %bind_addr, "server listening");
    axum::serve(listener, app).await?;

    Ok(())
}

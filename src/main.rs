mod api;
mod application;
mod domain;
mod infrastructure;

use std::{net::SocketAddr, sync::Arc};

use application::{api_key_service::ApiKeyService, user_service::UserService};
use infrastructure::persistence::{
    database::connect_and_migrate,
    repositories::{
        api_key_repository::SeaOrmApiKeyRepository, user_repository::SeaOrmUserRepository,
    },
};
use sea_orm::DbErr;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = connect_and_migrate("sqlite://app.db?mode=rwc").await?;

    let user_repo = Arc::new(SeaOrmUserRepository::new(db.clone()));
    let api_key_repo = Arc::new(SeaOrmApiKeyRepository::new(db.clone()));

    let state = api::handlers::AppState {
        user_service: UserService::new(user_repo.clone()),
        api_key_service: ApiKeyService::new(api_key_repo, user_repo),
    };

    let app = api::routes::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");
    axum::serve(listener, app)
        .await
        .expect("failed to start server");

    Ok(())
}

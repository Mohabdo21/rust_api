use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection, r2d2::ConnectionManager, sql_query,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::{
    domain::api_key_secret::{hash_api_key_value, is_hashed_api_key_value},
    infrastructure::persistence::{error::PersistenceError, schema::api_keys},
};

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn connect_and_migrate(database_url: &str) -> Result<DbPool, PersistenceError> {
    let database_url = normalize_sqlite_url(database_url)?;

    tokio::task::spawn_blocking(move || {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder().build(manager)?;

        let mut conn = pool.get()?;
        sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn)?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|err| PersistenceError::Migration(err.to_string()))?;
        backfill_legacy_api_key_hashes(&mut conn)?;

        Ok(pool)
    })
    .await?
}

fn backfill_legacy_api_key_hashes(conn: &mut SqliteConnection) -> Result<(), PersistenceError> {
    let rows = api_keys::table
        .select((api_keys::id, api_keys::key_hash))
        .load::<(String, String)>(conn)?;

    for (id, stored_value) in rows {
        if is_hashed_api_key_value(&stored_value) {
            continue;
        }

        diesel::update(api_keys::table.find(id))
            .set(api_keys::key_hash.eq(hash_api_key_value(&stored_value)))
            .execute(conn)?;
    }

    Ok(())
}

fn normalize_sqlite_url(database_url: &str) -> Result<String, PersistenceError> {
    if database_url == "sqlite::memory:" {
        return Ok(database_url.to_string());
    }

    if let Some(path) = database_url.strip_prefix("sqlite://") {
        let path = path.split('?').next().unwrap_or(path);
        if path.is_empty() {
            return Err(PersistenceError::InvalidDatabaseUrl(
                database_url.to_string(),
            ));
        }
        return Ok(path.to_string());
    }

    if database_url.starts_with("sqlite:") {
        return Ok(database_url.to_string());
    }

    Ok(database_url.to_string())
}

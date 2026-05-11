use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    application::ports::ApiKeyRepository,
    domain::models::ApiKey,
    infrastructure::persistence::{
        database::DbPool,
        entities::api_key::{ApiKeyRow, NewApiKeyRow},
        error::PersistenceError,
        schema::api_keys,
    },
};

pub struct DieselApiKeyRepository {
    pool: DbPool,
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid, PersistenceError> {
    Uuid::parse_str(value).map_err(|source| PersistenceError::InvalidUuid {
        field: field.to_string(),
        source,
    })
}

impl DieselApiKeyRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn to_domain(row: ApiKeyRow) -> Result<ApiKey, PersistenceError> {
    Ok(ApiKey {
        id: parse_uuid(&row.id, "api_keys.id")?,
        user_id: parse_uuid(&row.user_id, "api_keys.user_id")?,
        key_value: row.key_value,
        label: row.label,
        revoked: row.revoked,
    })
}

#[async_trait]
impl ApiKeyRepository for DieselApiKeyRepository {
    async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        key_value: String,
        label: Option<String>,
    ) -> Result<ApiKey, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let id_str = id.to_string();
            let user_id_str = user_id.to_string();

            diesel::insert_into(api_keys::table)
                .values(&NewApiKeyRow {
                    id: &id_str,
                    user_id: &user_id_str,
                    key_value: &key_value,
                    label: label.as_deref(),
                    revoked: false,
                })
                .execute(&mut conn)?;

            let inserted = api_keys::table
                .find(&id_str)
                .select(ApiKeyRow::as_select())
                .first::<ApiKeyRow>(&mut conn)?;

            to_domain(inserted)
        })
        .await?
    }

    async fn list(&self) -> Result<Vec<ApiKey>, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let rows = api_keys::table
                .select(ApiKeyRow::as_select())
                .load::<ApiKeyRow>(&mut conn)?;

            rows.into_iter().map(to_domain).collect()
        })
        .await?
    }

    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let rows = api_keys::table
                .filter(api_keys::user_id.eq(user_id.to_string()))
                .select(ApiKeyRow::as_select())
                .load::<ApiKeyRow>(&mut conn)?;

            rows.into_iter().map(to_domain).collect()
        })
        .await?
    }

    async fn set_revoked(
        &self,
        id: Uuid,
        revoked: bool,
    ) -> Result<Option<ApiKey>, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let id_str = id.to_string();

            let affected = diesel::update(api_keys::table.find(&id_str))
                .set(api_keys::revoked.eq(revoked))
                .execute(&mut conn)?;

            if affected == 0 {
                return Ok(None);
            }

            let updated = api_keys::table
                .find(&id_str)
                .select(ApiKeyRow::as_select())
                .first::<ApiKeyRow>(&mut conn)?;

            Ok(Some(to_domain(updated)?))
        })
        .await?
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<u64, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let affected =
                diesel::delete(api_keys::table.find(id.to_string())).execute(&mut conn)?;
            Ok(affected as u64)
        })
        .await?
    }
}

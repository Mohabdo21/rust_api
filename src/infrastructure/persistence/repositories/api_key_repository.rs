use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

use crate::{
    application::ports::ApiKeyRepository, domain::models::ApiKey,
    infrastructure::persistence::entities::api_key,
};

pub struct SeaOrmApiKeyRepository {
    db: DatabaseConnection,
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid, DbErr> {
    Uuid::parse_str(value).map_err(|err| DbErr::Custom(format!("invalid uuid in {field}: {err}")))
}

impl SeaOrmApiKeyRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ApiKeyRepository for SeaOrmApiKeyRepository {
    async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        key_value: String,
        label: Option<String>,
    ) -> Result<ApiKey, sea_orm::DbErr> {
        let active = api_key::ActiveModel {
            id: Set(id.to_string()),
            user_id: Set(user_id.to_string()),
            key_value: Set(key_value),
            label: Set(label),
            revoked: Set(false),
        };

        api_key::Entity::insert(active).exec(&self.db).await?;

        let inserted = api_key::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or_else(|| DbErr::Custom("inserted api key not found".to_string()))?;

        Ok(ApiKey {
            id: parse_uuid(&inserted.id, "api_keys.id")?,
            user_id: parse_uuid(&inserted.user_id, "api_keys.user_id")?,
            key_value: inserted.key_value,
            label: inserted.label,
            revoked: inserted.revoked,
        })
    }

    async fn list(&self) -> Result<Vec<ApiKey>, sea_orm::DbErr> {
        let models = api_key::Entity::find().all(&self.db).await?;
        models
            .into_iter()
            .map(|m| {
                Ok(ApiKey {
                    id: parse_uuid(&m.id, "api_keys.id")?,
                    user_id: parse_uuid(&m.user_id, "api_keys.user_id")?,
                    key_value: m.key_value,
                    label: m.label,
                    revoked: m.revoked,
                })
            })
            .collect()
    }

    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, sea_orm::DbErr> {
        let models = api_key::Entity::find()
            .filter(api_key::Column::UserId.eq(user_id.to_string()))
            .all(&self.db)
            .await?;

        models
            .into_iter()
            .map(|m| {
                Ok(ApiKey {
                    id: parse_uuid(&m.id, "api_keys.id")?,
                    user_id: parse_uuid(&m.user_id, "api_keys.user_id")?,
                    key_value: m.key_value,
                    label: m.label,
                    revoked: m.revoked,
                })
            })
            .collect()
    }

    async fn set_revoked(&self, id: Uuid, revoked: bool) -> Result<Option<ApiKey>, sea_orm::DbErr> {
        let maybe_model = api_key::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?;
        let Some(model) = maybe_model else {
            return Ok(None);
        };

        let mut active: api_key::ActiveModel = model.into();
        active.revoked = Set(revoked);
        let updated = active.update(&self.db).await?;

        Ok(Some(ApiKey {
            id: parse_uuid(&updated.id, "api_keys.id")?,
            user_id: parse_uuid(&updated.user_id, "api_keys.user_id")?,
            key_value: updated.key_value,
            label: updated.label,
            revoked: updated.revoked,
        }))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<u64, sea_orm::DbErr> {
        let result = api_key::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }
}

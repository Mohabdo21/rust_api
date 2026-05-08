use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    application::ports::ApiKeyRepository, domain::models::ApiKey,
    infrastructure::persistence::entities::api_key,
};

pub struct SeaOrmApiKeyRepository {
    db: DatabaseConnection,
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
        user_id: i32,
        key_value: String,
        label: Option<String>,
    ) -> Result<ApiKey, sea_orm::DbErr> {
        let inserted = api_key::ActiveModel {
            user_id: Set(user_id),
            key_value: Set(key_value),
            label: Set(label),
            revoked: Set(false),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(ApiKey {
            id: inserted.id,
            user_id: inserted.user_id,
            key_value: inserted.key_value,
            label: inserted.label,
            revoked: inserted.revoked,
        })
    }

    async fn list(&self) -> Result<Vec<ApiKey>, sea_orm::DbErr> {
        let models = api_key::Entity::find().all(&self.db).await?;
        Ok(models
            .into_iter()
            .map(|m| ApiKey {
                id: m.id,
                user_id: m.user_id,
                key_value: m.key_value,
                label: m.label,
                revoked: m.revoked,
            })
            .collect())
    }

    async fn list_by_user(&self, user_id: i32) -> Result<Vec<ApiKey>, sea_orm::DbErr> {
        let models = api_key::Entity::find()
            .filter(api_key::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        Ok(models
            .into_iter()
            .map(|m| ApiKey {
                id: m.id,
                user_id: m.user_id,
                key_value: m.key_value,
                label: m.label,
                revoked: m.revoked,
            })
            .collect())
    }

    async fn set_revoked(&self, id: i32, revoked: bool) -> Result<Option<ApiKey>, sea_orm::DbErr> {
        let maybe_model = api_key::Entity::find_by_id(id).one(&self.db).await?;
        let Some(model) = maybe_model else {
            return Ok(None);
        };

        let mut active: api_key::ActiveModel = model.into();
        active.revoked = Set(revoked);
        let updated = active.update(&self.db).await?;

        Ok(Some(ApiKey {
            id: updated.id,
            user_id: updated.user_id,
            key_value: updated.key_value,
            label: updated.label,
            revoked: updated.revoked,
        }))
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, sea_orm::DbErr> {
        let result = api_key::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected)
    }
}

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use uuid::Uuid;

use crate::{
    application::ports::UserRepository, domain::models::User,
    infrastructure::persistence::entities::user,
};

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

fn parse_uuid(value: &str) -> Result<Uuid, DbErr> {
    Uuid::parse_str(value).map_err(|err| DbErr::Custom(format!("invalid uuid in users.id: {err}")))
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, id: Uuid, name: String, email: String) -> Result<User, sea_orm::DbErr> {
        let inserted = user::ActiveModel {
            id: Set(id.to_string()),
            name: Set(name),
            email: Set(email),
        }
        .insert(&self.db)
        .await?;

        Ok(User {
            id: parse_uuid(&inserted.id)?,
            name: inserted.name,
            email: inserted.email,
        })
    }

    async fn list(&self) -> Result<Vec<User>, sea_orm::DbErr> {
        let models = user::Entity::find().all(&self.db).await?;
        models
            .into_iter()
            .map(|m| {
                Ok(User {
                    id: parse_uuid(&m.id)?,
                    name: m.name,
                    email: m.email,
                })
            })
            .collect()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sea_orm::DbErr> {
        let model = user::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?;
        model
            .map(|m| {
                Ok(User {
                    id: parse_uuid(&m.id)?,
                    name: m.name,
                    email: m.email,
                })
            })
            .transpose()
    }
}

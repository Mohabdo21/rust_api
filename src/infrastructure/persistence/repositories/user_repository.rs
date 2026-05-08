use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    application::ports::UserRepository, domain::models::User,
    infrastructure::persistence::entities::user,
};

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, name: String, email: String) -> Result<User, sea_orm::DbErr> {
        let inserted = user::ActiveModel {
            name: Set(name),
            email: Set(email),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(User {
            id: inserted.id,
            name: inserted.name,
            email: inserted.email,
        })
    }

    async fn list(&self) -> Result<Vec<User>, sea_orm::DbErr> {
        let models = user::Entity::find().all(&self.db).await?;
        Ok(models
            .into_iter()
            .map(|m| User {
                id: m.id,
                name: m.name,
                email: m.email,
            })
            .collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sea_orm::DbErr> {
        let model = user::Entity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| User {
            id: m.id,
            name: m.name,
            email: m.email,
        }))
    }
}

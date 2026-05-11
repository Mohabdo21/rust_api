use async_trait::async_trait;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    application::ports::UserRepository,
    domain::models::User,
    infrastructure::persistence::{
        database::DbPool,
        entities::user::{NewUserRow, UserRow},
        error::PersistenceError,
        schema::users,
    },
};

pub struct DieselUserRepository {
    pool: DbPool,
}

fn parse_uuid(value: &str) -> Result<Uuid, PersistenceError> {
    Uuid::parse_str(value).map_err(|source| PersistenceError::InvalidUuid {
        field: "users.id".to_string(),
        source,
    })
}

impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn to_domain(row: UserRow) -> Result<User, PersistenceError> {
    Ok(User {
        id: parse_uuid(&row.id)?,
        name: row.name,
        email: row.email,
    })
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn create(
        &self,
        id: Uuid,
        name: String,
        email: String,
    ) -> Result<User, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let id_str = id.to_string();

            diesel::insert_into(users::table)
                .values(&NewUserRow {
                    id: &id_str,
                    name: &name,
                    email: &email,
                })
                .execute(&mut conn)?;

            let inserted = users::table
                .find(&id_str)
                .select(UserRow::as_select())
                .first::<UserRow>(&mut conn)?;

            to_domain(inserted)
        })
        .await?
    }

    async fn list(&self) -> Result<Vec<User>, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let rows = users::table
                .select(UserRow::as_select())
                .load::<UserRow>(&mut conn)?;

            rows.into_iter().map(to_domain).collect()
        })
        .await?
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<u64, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let affected = diesel::delete(users::table.find(id.to_string())).execute(&mut conn)?;
            Ok(affected as u64)
        })
        .await?
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, PersistenceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            let maybe_row = users::table
                .find(id.to_string())
                .select(UserRow::as_select())
                .first::<UserRow>(&mut conn)
                .optional()?;

            maybe_row.map(to_domain).transpose()
        })
        .await?
    }
}

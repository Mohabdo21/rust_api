use diesel::prelude::*;

use crate::infrastructure::persistence::{entities::user::UserRow, schema::api_keys};

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = api_keys)]
#[diesel(belongs_to(UserRow, foreign_key = user_id))]
pub struct ApiKeyRow {
    pub id: String,
    pub user_id: String,
    pub key_hash: String,
    pub label: Option<String>,
    pub revoked: bool,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKeyRow<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub key_hash: &'a str,
    pub label: Option<&'a str>,
    pub revoked: bool,
}

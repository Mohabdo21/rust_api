use diesel::prelude::*;

use crate::infrastructure::persistence::schema::users;

#[derive(Clone, Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserRow<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}

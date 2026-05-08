use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_value: String,
    pub label: Option<String>,
    pub revoked: bool,
}

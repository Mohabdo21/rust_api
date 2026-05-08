#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct ApiKey {
    pub id: i32,
    pub user_id: i32,
    pub key_value: String,
    pub label: Option<String>,
    pub revoked: bool,
}

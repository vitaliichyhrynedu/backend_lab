use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Serialize, Deserialize)]
pub struct UsersBody<T> {
    pub users: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub name: String,
}

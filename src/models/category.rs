use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CategoryBody<T> {
    pub category: T,
}

#[derive(Serialize, Deserialize)]
pub struct CategoriesBody<T> {
    pub categories: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CategoryCreate {
    pub name: String,
}

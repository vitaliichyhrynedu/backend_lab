use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use uuid::Uuid;

use crate::{AppState, database::Table, models::category::*};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route("/{category_id}", get(get_category).delete(delete_category))
}

async fn get_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<CategoryBody<Category>> {
    let db = db.read().await;
    let table = match db.get("categories") {
        Some(Table::Categories(table)) => table,
        _ => panic!("'categories' table not found or has wrong type"),
    };
    let category = table.get(&id).unwrap().clone();
    Json(CategoryBody { category })
}

async fn create_category(
    State(AppState { db }): State<AppState>,
    Json(CategoryBody { category }): Json<CategoryBody<CategoryCreate>>,
) -> (StatusCode, Json<CategoryBody<Category>>) {
    let mut db = db.write().await;
    let table = match db.get_mut("categories") {
        Some(Table::Categories(table)) => table,
        _ => panic!("'categories' table not found or has wrong type"),
    };
    let category = Category {
        id: Uuid::new_v4(),
        name: category.name,
    };
    table.insert(category.id, category.clone());
    (StatusCode::CREATED, Json(CategoryBody { category }))
}

async fn delete_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut db = db.write().await;
    let table = match db.get_mut("categories") {
        Some(Table::Categories(table)) => table,
        _ => panic!("'categories' table not found or has wrong type"),
    };
    table.remove(&id);
    StatusCode::NO_CONTENT
}

async fn get_categories(State(AppState { db }): State<AppState>) -> Json<CategoriesBody<Category>> {
    let db = db.read().await;
    let table = match db.get("categories") {
        Some(Table::Categories(table)) => table,
        _ => panic!("'categories' table not found or has wrong type"),
    };
    let categories = table.values().cloned().collect();
    Json(CategoriesBody { categories })
}

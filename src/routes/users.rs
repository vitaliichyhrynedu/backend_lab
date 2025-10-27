use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use uuid::Uuid;

use crate::{AppState, database::Table, models::user::*};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/{user_id}", get(get_user).delete(delete_user))
}

async fn get_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<UserBody<User>> {
    let db = db.read().await;
    let table = match db.get("users") {
        Some(Table::Users(table)) => table,
        _ => panic!("'users' table not found or has wrong type"),
    };
    let user = table.get(&id).unwrap().clone();
    Json(UserBody { user })
}

async fn create_user(
    State(AppState { db }): State<AppState>,
    Json(UserBody { user }): Json<UserBody<UserCreate>>,
) -> (StatusCode, Json<UserBody<User>>) {
    let mut db = db.write().await;
    let table = match db.get_mut("users") {
        Some(Table::Users(table)) => table,
        _ => panic!("'users' table not found or has wrong type"),
    };
    let user = User {
        id: Uuid::new_v4(),
        name: user.name,
    };
    table.insert(user.id, user.clone());
    (StatusCode::CREATED, Json(UserBody { user }))
}

async fn delete_user(State(AppState { db }): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut db = db.write().await;
    let table = match db.get_mut("users") {
        Some(Table::Users(table)) => table,
        _ => panic!("'users' table not found or has wrong type"),
    };
    table.remove(&id);
    StatusCode::NO_CONTENT
}

async fn get_users(State(AppState { db }): State<AppState>) -> Json<UsersBody<User>> {
    let db = db.read().await;
    let table = match db.get("users") {
        Some(Table::Users(table)) => table,
        _ => panic!("'users' table not found or has wrong type"),
    };
    let users = table.values().cloned().collect();
    Json(UsersBody { users })
}

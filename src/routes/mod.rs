use axum::{Router, routing::get};

use crate::AppState;

mod categories;
mod health;
mod users;

pub fn router() -> Router<AppState> {
    let health = health::router();
    let users = users::router();
    let categories = categories::router();

    Router::new()
        .route("/", get(get_root))
        .nest("/health", health)
        .nest("/users", users)
        .nest("/categories", categories)
}

async fn get_root() -> &'static str {
    "Welcome to the expense tracker application."
}

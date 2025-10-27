use axum::{Router, routing::get};

use crate::AppState;

mod health;

pub fn router() -> Router<AppState> {
    let health = health::router();

    Router::new()
        .route("/", get(get_root))
        .nest("/health", health)
}

async fn get_root() -> &'static str {
    "Welcome to the expense tracker application."
}

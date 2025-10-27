use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(get_root))
}

async fn get_root() -> &'static str {
    "Welcome to the expense tracker application."
}

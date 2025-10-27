use axum::{Json, Router, http::StatusCode, routing::get};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct Health {
    status: Status,
    observed_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Up,
    Down,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_health))
}

pub async fn get_health() -> (StatusCode, Json<Health>) {
    let health = Health {
        status: Status::Up,
        observed_at: Utc::now(),
    };
    (StatusCode::OK, Json(health))
}

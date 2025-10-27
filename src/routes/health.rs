use axum::{Router, extract::State, routing::get};
use chrono::Utc;
use sea_orm::DatabaseConnection;

use crate::{AppState, models::health::*};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_health))
}

async fn get_health(State(state): State<AppState>) -> Health {
    let services = Services {
        db: check_db(&state.db).await,
    };
    Health {
        status: services.health(),
        observed_at: Utc::now(),
        services,
    }
}

async fn check_db(db: &DatabaseConnection) -> Status {
    match db.ping().await {
        Ok(_) => Status::Up,
        Err(e) => {
            eprintln!("database ping failed: {e}");
            Status::Down
        }
    }
}

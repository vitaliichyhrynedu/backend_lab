use std::env;

use sea_orm::{Database, DatabaseConnection};

mod models;
mod routes;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url)
        .await
        .expect("database connection failed");
    eprintln!("database connection established");

    let state = AppState { db };
    let router = routes::router().with_state(state);

    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("listening on http://localhost:{port}");

    axum::serve(listener, router).await.unwrap();
}

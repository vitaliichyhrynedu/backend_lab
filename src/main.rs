use std::env;

use crate::database::Database;

mod database;
mod models;
mod routes;

#[derive(Clone)]
struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = database::create().await;
    let state = AppState { db };
    let router = routes::router().with_state(state);

    let port = env::var("PORT").expect("PORT must be set");
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    eprintln!("listening on http://localhost:{port}");

    axum::serve(listener, router).await.unwrap();
}

use std::env;

mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let router = routes::router();

    let port = env::var("PORT").expect("PORT must be set");
    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

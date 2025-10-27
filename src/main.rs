mod routes;

#[tokio::main]
async fn main() {
    let router = routes::router();
    let address = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

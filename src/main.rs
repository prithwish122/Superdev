use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handler() -> &'static str {
    "Hey this is a test"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}

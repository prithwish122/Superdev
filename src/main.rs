use axum::{Router, routing::{get, post}, serve, Json};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handler() -> &'static str {
    "Hey this is a test"
}

async fn keypair_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "pubkey": "base58-encoded-public-key",
            "secret": "base58-encoded-secret-key"
        }
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/keypair", post(keypair_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}

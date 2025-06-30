use axum::{
    routing::{get, post},
    Json, Router,
};
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

async fn createspl() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "program_id": "string",
            "accounts": [
                {
                    "pubkey": "pubkey",
                    "is_signer": true,
                    "is_writable": false
                },
                {
                    "pubkey": "another-pubkey",
                    "is_signer": false,
                    "is_writable": true
                }
            ],
            "instruction_data": "base64-encoded-data"
        }
    }))
}

async fn mint_to_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "program_id": "string",
            "accounts": [
                {
                    "pubkey": "mint-address",
                    "is_signer": false,
                    "is_writable": true
                },
                {
                    "pubkey": "destination-user-address",
                    "is_signer": false,
                    "is_writable": true
                },
                {
                    "pubkey": "authority-address",
                    "is_signer": true,
                    "is_writable": false
                }
            ],
            "instruction_data": "base64-encoded-data"
        }
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/keypair", post(keypair_handler))
        .route("/token/create", post(createspl))
        .route("/token/mint", post(mint_to_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::Value;

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

async fn createspl(Json(body): Json<Value>) -> Json<serde_json::Value> {
    println!("Received body: {:?}", body);

    if body.get("mintAuthority").is_none() || body.get("mint").is_none() || body.get("decimals").is_none() {
        return Json(json!({
            "success": false,
            "error": "Missing required fields: mintAuthority, mint, or decimals"
        }));
    }

    // let has_name = body.get("name").is_some();

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


async fn mint_to_handler(Json(body): Json<Value>) -> Json<serde_json::Value> {
    println!("Received body: {:?}", body);

    if body.get("mint").is_none() || body.get("destination").is_none() || body.get("authority").is_none() || body.get("amount").is_none() {
        return Json(json!({
            "success": false,
            "error": "Missing required fields: mintAuthority, mint, or decimals"
        }));
    }
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




async fn message_sign_handler(Json(body): Json<Value>) -> Json<serde_json::Value> {
    println!("Received body: {:?}", body);

    if body.get("message").is_none() || body.get("secret").is_none() {
        return Json(json!({
            "success": false,
            "error": "Missing required fields: message or signer"
        }));
    }

    Json(json!({
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/keypair", post(keypair_handler))
        .route("/token/create", post(createspl))
        .route("/token/mint", post(mint_to_handler))
        .route("/message/sign", post(message_sign_handler));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

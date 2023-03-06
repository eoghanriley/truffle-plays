use std::sync::{Arc, Mutex};

use axum::{extract, extract::State, routing::post, Json, Router};
use rustis::{client::Client, commands::ListCommands};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Viewer {
    org_id: String,
    input: String,
    stream: String,
}

#[derive(Deserialize, Debug)]
struct StreamerReq {
    org_id: String,
    password: String,
    stream: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct StreamerRes {
    input: String,
}

struct AppState {
    redis_client: Client,
}

async fn shift(
    State(mut state): State<Client>,
    extract::Json(payload): extract::Json<StreamerReq>,
) {
    let e: Vec<String> = state.rpop(payload.stream, 10).await.unwrap();

    println!("{:?}", e);
}

#[axum_macros::debug_handler]
async fn push(State(mut state): State<Client>, extract::Json(payload): extract::Json<Viewer>) {
    let _ = state.lpush(payload.stream, payload.input).await.unwrap();
}

#[tokio::main]
async fn main() {
    let mut client = Client::connect("redis://127.0.0.1:7001").await.unwrap();

    let app = Router::new()
        .route("/shift", post(shift))
        .route("/push", post(push))
        .with_state(client);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{extract, routing::post, Json, Router};
use redis::Commands;
use redis::RedisResult;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

#[derive(Deserialize, Debug)]
struct Body {
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

async fn shift(extract::Json(payload): extract::Json<StreamerReq>) -> Json<StreamerRes> {
    let client = redis::Client::open("redis://127.0.0.1:7001").unwrap();
    let mut con = client.get_connection().unwrap();

    let res = StreamerRes {
        input: redis::cmd("RPOP")
            .arg(payload.stream)
            .query(&mut con)
            .unwrap(),
    };

    Json(res)
}

async fn push(extract::Json(payload): extract::Json<Body>) {
    let client = redis::Client::open("redis://127.0.0.1:7001").unwrap();
    let mut con = client.get_connection().unwrap();

    let _: () = con.lpush(payload.stream, payload.input).unwrap();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/shift", post(shift))
        .route("/push", post(push));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

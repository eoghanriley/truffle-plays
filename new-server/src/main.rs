use axum::{extract, extract::State, routing::get, routing::post, Json, Router};
use rustis::{client::Client, commands::ListCommands, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;
use tokio::fs::read_to_string;

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
    input: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Streamer {
    id: Option<i32>,
    username: String,
    password: String,
    stream: String,
    api_token: Option<String>,
    donater: Option<bool>,
    active_stream: Option<bool>,
}

#[derive(Clone)]
struct AppState {
    redis_client: Client,
    db_pool: SqlitePool,
}

async fn shift(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<StreamerReq>,
) -> Json<StreamerRes> {
    let res: Vec<String> = state.redis_client.rpop(payload.stream, 10).await.unwrap();
    Json(StreamerRes { input: res })
}

#[axum_macros::debug_handler]
async fn push(State(mut state): State<AppState>, extract::Json(payload): extract::Json<Viewer>) {
    let _ = state
        .redis_client
        .lpush(payload.stream, payload.input)
        .await
        .unwrap();
}

#[axum_macros::debug_handler]
async fn register_streamer(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<Streamer>,
) {
    sqlx::query(
        "INSERT INTO streamers (username, password, stream, api_token, donater) VALUES (?, ?, ?, ?, ?)"
    )
        .bind(payload.username)
        .bind(payload.password).bind(payload.stream)
        .bind(payload.api_token)
        .bind(payload.donater)
        .execute(&state.db_pool).await.unwrap();
}

async fn get_streamer(State(state): State<AppState>) -> Json<Streamer> {
    let x = sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE id = ?")
        .bind(1)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    Json(x)
}

async fn migrate(db_pool: &SqlitePool) {
    let schema = read_to_string("src/schema.sql");
    sqlx::query(&schema.await.unwrap())
        .execute(db_pool)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let redis_client = Client::connect(redis_url).await?;
    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite://sqlite.db")
        .await
        .unwrap();

    migrate(&db_pool).await;

    let app = Router::new()
        .route("/shift", post(shift))
        .route("/push", post(push))
        .route("/register", post(register_streamer))
        .route("/get", get(get_streamer))
        .with_state(AppState {
            redis_client,
            db_pool,
        });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

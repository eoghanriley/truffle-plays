use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, Salt, SaltString},
    Argon2,
};
use axum::{extract, extract::State, routing::post, Json, Router};
use rustis::{client::Client, commands::ListCommands, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;
use tokio::fs::read_to_string;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Viewer {
    org_id: String,
    input: String,
    stream: String,
}

#[derive(Deserialize, Debug)]
struct StreamerReq {
    api_token: String,
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

#[derive(Deserialize, Serialize, Debug)]
struct AppRes<'a, T> {
    body: Option<T>,
    error: Option<&'a str>,
}

// TODO: change unwraps to expect

async fn shift(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<StreamerReq>,
) -> Json<AppRes<'static, Vec<String>>> {
    let streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE api_token = ? LIMIT 1")
            .bind(payload.api_token)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    // TODO: Add check if stream is active

    let res: Vec<String> = state.redis_client.rpop(streamer.stream, 10).await.unwrap();
    Json(AppRes {
        body: Some(res),
        error: None,
    })
}

#[axum_macros::debug_handler]
async fn push(State(mut state): State<AppState>, extract::Json(payload): extract::Json<Viewer>) {
    // TODO: Add check if stream is active

    let _ = state
        .redis_client
        .lpush(payload.stream, payload.input)
        .await
        .unwrap();
}

fn gen_uuid() -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_OID, Uuid::new_v4().as_bytes())
}

#[axum_macros::debug_handler]
async fn register_streamer(
    State(state): State<AppState>,
    extract::Json(mut payload): extract::Json<Streamer>,
) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    payload.password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    payload.api_token = Some(gen_uuid().to_string());

    sqlx::query(
        "INSERT INTO streamers (username, password, stream, api_token) VALUES (?, ?, ?, ?)",
    )
    .bind(payload.username)
    .bind(payload.password)
    .bind(payload.stream)
    .bind(payload.api_token)
    .execute(&state.db_pool)
    .await
    .unwrap();
}

fn verify_pass(hash: &str, pass: &str) -> bool {
    let salt = hash.split("$").collect::<Vec<&str>>()[4];

    let pass = Argon2::default()
        .hash_password(pass.as_bytes(), Salt::from_b64(salt).unwrap())
        .unwrap();

    if pass.to_string() == hash {
        return true;
    }

    return false;
}

#[axum_macros::debug_handler]
async fn regen_token(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<Streamer>,
) -> Json<AppRes<'static, String>> {
    let streamer = sqlx::query_as::<_, Streamer>(
        "SELECT * FROM streamers WHERE username = ? AND stream = ? LIMIT 1",
    )
    .bind(payload.username)
    .bind(payload.stream)
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    if verify_pass(&streamer.password, &payload.password) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with password validation"),
        });
    }

    let new_token = gen_uuid().to_string();
    sqlx::query("UPDATE streamers SET api_token = ? WHERE id = ? RETURNING api_token")
        .bind(&new_token)
        .bind(streamer.id.unwrap())
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    Json(AppRes {
        body: Some(new_token),
        error: None,
    })
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
        .route("/regen_token", post(regen_token))
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

mod auth;
mod util;
mod viewer;

use auth::{login, regen_token, register_streamer};
use axum::{
    extract,
    extract::State,
    http::{header, Method},
    routing::get,
    routing::post,
    Json, Router,
};
use rustis::{client::Client, commands::GenericCommands, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use util::verify_hash;
use viewer::{get_active_streamers, push, shift};

#[derive(Deserialize, Debug)]
pub struct Viewer {
    user_id: String,
    input: String,
    stream: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct StreamerRes {
    input: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Streamer {
    id: Option<i32>,
    org_id: String,
    password: String,
    stream: String,
    api_token: Option<String>,
    donater: Option<bool>,
    active_stream: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct RegisterLinks {
    link: String,
    used: bool,
}

#[derive(Clone)]
pub struct AppState {
    redis_client: Client,
    db_pool: PgPool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppRes<'a, T> {
    body: Option<T>,
    error: Option<&'a str>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppReq {
    api_token: String,
    org_id: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Stream {
    stream: String,
    org_id: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Streams {
    names: Vec<Stream>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    org_id: String,
    password: String,
}

async fn toggle_stream(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, bool>> {
    let mut streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE org_id = $1 LIMIT 1")
            .bind(&payload.org_id)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    if verify_hash(&streamer.api_token.unwrap(), &payload.api_token) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    streamer.active_stream = Some(!streamer.active_stream.unwrap());

    sqlx::query(
        "UPDATE streamers SET active_stream = $1 WHERE org_id = $2 RETURNING active_stream",
    )
    .bind(&streamer.active_stream.unwrap())
    .bind(payload.org_id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    if &streamer.active_stream.unwrap() == &false {
        let _ = state.redis_client.del(streamer.stream);
    }

    Json(AppRes {
        body: Some(!streamer.active_stream.unwrap()),
        error: None,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let redis_client = Client::connect(redis_url).await?;
    let db_pool = PgPoolOptions::new()
        .max_connections(300)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();

    sqlx::migrate!().run(&db_pool).await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
        .allow_headers([
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ])
        .allow_origin(Any);
    let app = Router::new()
        .route("/shift", post(shift))
        .route("/push", post(push))
        .route("/register/:link", post(register_streamer))
        .route("/regen_token", post(regen_token))
        .route("/toggle_stream", post(toggle_stream))
        .route("/login", post(login))
        .route("/get_streams", get(get_active_streamers))
        .with_state(AppState {
            redis_client,
            db_pool,
        })
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

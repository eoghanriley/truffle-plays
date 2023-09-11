mod auth;
mod db;
mod streamer;
mod util;
mod viewer;

use crate::{db::Stream, util::AppReq, util::AppRes};
use auth::{login, regen_token, register_streamer};
use axum::{
    http::{header, Method},
    routing::get,
    routing::post,
    Router,
};
use rustis::{client::Client, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use streamer::toggle_stream;
use tower_http::cors::{Any, CorsLayer};
use util::verify_hash;
use viewer::{get_active_streamers, push, shift};

#[derive(Clone)]
pub struct AppState {
    redis_client: Client,
    db_pool: PgPool,
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

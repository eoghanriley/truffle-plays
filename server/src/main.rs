mod auth;
mod util;

use auth::{login, regen_token, register_streamer};
use axum::{
    extract,
    extract::State,
    http::{header, Method},
    routing::get,
    routing::post,
    Json, Router,
};
use chrono::{DateTime, Utc};
use rustis::{
    client::Client,
    commands::StringCommands,
    commands::{GenericCommands, ListCommands},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use util::verify_hash;

#[derive(Deserialize, Debug)]
struct Viewer {
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
struct AppReq {
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

// TODO: change unwraps to expect

async fn shift(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, Vec<String>>> {
    let streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE org_id = $1 LIMIT 1")
            .bind(payload.org_id)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    if verify_hash(&streamer.api_token.unwrap(), &payload.api_token) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    if streamer.active_stream.unwrap() == false {
        return Json(AppRes {
            body: None,
            error: Some("Stream not active"),
        });
    }

    let res: Vec<String> = state.redis_client.rpop(streamer.stream, 10).await.unwrap();
    Json(AppRes {
        body: Some(res),
        error: None,
    })
}

#[axum_macros::debug_handler]
async fn push(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<Viewer>,
) -> Json<AppRes<'static, &'static str>> {
    let last_push: Option<String> = state
        .redis_client
        .get(payload.user_id.clone())
        .await
        .unwrap();

    if last_push.is_some() {
        if (Utc::now().time()
            - last_push
                .clone()
                .unwrap()
                .parse::<DateTime<Utc>>()
                .unwrap()
                .time())
        .num_milliseconds()
            < 1000
        {
            return Json(AppRes {
                body: None,
                error: Some(
                    "Slow down you are sending too many inputs! \nYou can only send 1 a second.",
                ),
            });
        }
    }

    let _ = state
        .redis_client
        .set(payload.user_id, Utc::now().to_string())
        .await
        .unwrap();

    let streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE stream = $1 LIMIT 1")
            .bind(&payload.stream)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    if streamer.active_stream.unwrap() == false {
        return Json(AppRes {
            body: None,
            error: Some("Stream not active"),
        });
    }

    let _ = state
        .redis_client
        .lpush(payload.stream, payload.input)
        .await
        .unwrap();

    Json(AppRes {
        body: Some("Success"),
        error: None,
    })
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

async fn get_active_streamers(State(state): State<AppState>) -> extract::Json<Streams> {
    Json(Streams {
        names: sqlx::query_as::<_, Stream>(
            "SELECT stream, org_id FROM streamers where active_stream = True",
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap(),
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

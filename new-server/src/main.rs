use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, Salt, SaltString},
    Argon2,
};
use axum::{extract, extract::State, routing::get, routing::post, Json, Router};
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
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Viewer {
    org_id: String,
    input: String,
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
    db_pool: PgPool,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppRes<'a, T> {
    body: Option<T>,
    error: Option<&'a str>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppReq {
    api_token: String,
    username: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Stream {
    stream: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Streams {
    names: Vec<Stream>,
}

// TODO: change unwraps to expect

async fn shift(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, Vec<String>>> {
    let streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE username = $1 LIMIT 1")
            .bind(payload.username)
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
        .get(payload.org_id.clone())
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
                error: Some("Slow down. Too many requests"),
            });
        }
    }

    let _ = state
        .redis_client
        .set(payload.org_id, Utc::now().to_string())
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

fn gen_uuid() -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_OID, Uuid::new_v4().as_bytes())
}

#[axum_macros::debug_handler]
async fn register_streamer(
    State(state): State<AppState>,
    extract::Json(mut payload): extract::Json<Streamer>,
) -> extract::Json<AppRes<'static, String>> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    payload.password = argon2
        .hash_password(payload.password.as_bytes(), &password_salt)
        .unwrap()
        .to_string();

    let unhashed_token = gen_uuid().to_string();
    let token_salt = SaltString::generate(&mut OsRng);

    let hashed_token = argon2
        .hash_password(&unhashed_token.as_bytes(), &token_salt)
        .unwrap()
        .to_string();

    payload.api_token = Some(gen_uuid().to_string());

    sqlx::query(
        "INSERT INTO streamers (username, password, stream, api_token) VALUES ($1, $2, $3, $4)",
    )
    .bind(payload.username)
    .bind(payload.password)
    .bind(payload.stream)
    .bind(hashed_token)
    .execute(&state.db_pool)
    .await
    .unwrap();

    Json(AppRes {
        body: Some(unhashed_token),
        error: None,
    })
}

fn verify_hash(hashed_value: &str, unhashed_value: &str) -> bool {
    let salt = hashed_value.split("$").collect::<Vec<&str>>()[4];

    let pass = Argon2::default()
        .hash_password(unhashed_value.as_bytes(), Salt::from_b64(salt).unwrap())
        .unwrap();

    if pass.to_string() == hashed_value {
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
        "SELECT * FROM streamers WHERE username = $1 AND stream = $2 LIMIT 1",
    )
    .bind(payload.username)
    .bind(payload.stream)
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    if verify_hash(&streamer.password, &payload.password) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with password validation"),
        });
    }

    let unhashed_token = gen_uuid().to_string();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_token = argon2
        .hash_password(&unhashed_token.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query("UPDATE streamers SET api_token = $1 WHERE id = $2 RETURNING api_token")
        .bind(hashed_token)
        .bind(streamer.id.unwrap())
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    Json(AppRes {
        body: Some(unhashed_token),
        error: None,
    })
}

async fn toggle_stream(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, bool>> {
    let mut streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE username = $1 LIMIT 1")
            .bind(&payload.username)
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
        "UPDATE streamers SET active_stream = $1 WHERE username = $2 RETURNING active_stream",
    )
    .bind(&streamer.active_stream.unwrap())
    .bind(payload.username)
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

async fn login(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> extract::Json<AppRes<'static, Streamer>> {
    let mut streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE api_token = $1 LIMIT 1")
            .bind(&payload.api_token)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    streamer.id = None;
    streamer.password = "".to_string();
    streamer.api_token = None;
    streamer.donater = None;

    Json(AppRes {
        body: Some(streamer),
        error: None,
    })
}

async fn get_active_streamers(State(state): State<AppState>) -> extract::Json<Streams> {
    Json(Streams {
        names: sqlx::query_as::<_, Stream>(
            "SELECT stream FROM streamers where active_stream = True",
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

    let app = Router::new()
        .route("/shift", post(shift))
        .route("/push", post(push))
        .route("/register", post(register_streamer))
        .route("/regen_token", post(regen_token))
        .route("/toggle_stream", post(toggle_stream))
        .route("/login", post(login))
        .route("/get_streams", get(get_active_streamers))
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

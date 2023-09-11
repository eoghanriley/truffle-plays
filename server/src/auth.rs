use crate::{db::RegisterLinks, db::Streamer, verify_hash, AppRes, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract, extract::Path, extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    org_id: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn register_streamer(
    State(state): State<AppState>,
    Path(link): Path<String>,
    extract::Json(mut payload): extract::Json<Streamer>,
) -> extract::Json<AppRes<'static, String>> {
    let valid = sqlx::query_as::<_, RegisterLinks>("SELECT * FROM register_links WHERE link = $1")
        .bind(link)
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or_else(|_| RegisterLinks {
            link: "".to_string(),
            used: true,
        });

    if valid.used == true || valid.link == "" {
        return Json(AppRes {
            body: None,
            error: Some("Sign up key is not valid."),
        });
    }

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
        "INSERT INTO streamers (org_id, password, stream, api_token) VALUES ($1, $2, $3, $4)",
    )
    .bind(payload.org_id)
    .bind(payload.password)
    .bind(payload.stream)
    .bind(hashed_token)
    .execute(&state.db_pool)
    .await
    .unwrap();

    sqlx::query("UPDATE register_links SET used = True WHERE link = $1")
        .bind(valid.link)
        .execute(&state.db_pool)
        .await
        .unwrap();

    Json(AppRes {
        body: Some(unhashed_token),
        error: None,
    })
}

pub fn gen_uuid() -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_OID, Uuid::new_v4().as_bytes())
}

#[axum_macros::debug_handler]
pub async fn regen_token(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<Streamer>,
) -> Json<AppRes<'static, String>> {
    let streamer = sqlx::query_as::<_, Streamer>(
        "SELECT * FROM streamers WHERE org_id = $1 AND stream = $2 LIMIT 1",
    )
    .bind(payload.org_id)
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

pub async fn login(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<Login>,
) -> extract::Json<AppRes<'static, bool>> {
    let streamer =
        sqlx::query_as::<_, Streamer>("SELECT * FROM streamers WHERE org_id = $1 LIMIT 1")
            .bind(&payload.org_id)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    if verify_hash(&streamer.password, &payload.password) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    Json(AppRes {
        body: streamer.active_stream,
        error: None,
    })
}

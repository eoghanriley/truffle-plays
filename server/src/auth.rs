use crate::{db::Mod, db::Org, db::RegisterLinks, verify_hash, AppRes, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract, extract::Path, extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    name: String,
    org_id: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterMod {
    name: String,
    password: String,
    org_id: String,
}

#[axum_macros::debug_handler]
pub async fn register_mod(
    State(state): State<AppState>,
    Path(link): Path<String>,
    extract::Json(mut payload): extract::Json<RegisterMod>,
) -> extract::Json<AppRes<'static, String>> {
    // Check if link is valid
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

    // Gen password
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    payload.password = argon2
        .hash_password(payload.password.as_bytes(), &password_salt)
        .unwrap()
        .to_string();

    // Gen api_token
    let unhashed_token = gen_uuid().to_string();
    let token_salt = SaltString::generate(&mut OsRng);

    let hashed_token = argon2
        .hash_password(&unhashed_token.as_bytes(), &token_salt)
        .unwrap()
        .to_string();
    let id = gen_uuid().to_string();

    let mut root = false;
    let mut receiver = false;

    let org_exists = sqlx::query!(r#"SELECT EXISTS (SELECT 1 FROM orgs WHERE org_id = $1) as exists"#, &payload.org_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap()
        .exists
        .unwrap();
    if !org_exists {
        root = true;
        receiver = true;

        sqlx::query(r#"INSERT INTO orgs (org_id, name) VALUES ($1, $2)"#)
            .bind(&payload.org_id)
            .bind(format!("{}'s org", &payload.name))
            .execute(&state.db_pool)
            .await
            .unwrap();
    }

    // Create user
    sqlx::query(
        "INSERT INTO mods (id, name, password, org_id, api_token, root, receiver) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(&id)
    .bind(payload.name)
    .bind(payload.password)
    .bind(payload.org_id)
    .bind(hashed_token)
    .bind(root)
    .bind(receiver)
    .execute(&state.db_pool)
    .await
    .unwrap();

    // Mark link as used
    sqlx::query("UPDATE register_links SET used = True, used_by = $1 WHERE link = $2")
        .bind(id)
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
    extract::Json(payload): extract::Json<Login>,
) -> Json<AppRes<'static, String>> {
    // Get user
    let user = sqlx::query_as::<_, Mod>("SELECT * FROM mods WHERE org_id = $1 LIMIT 1")
        .bind(payload.org_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    // Auth user
    if verify_hash(&user.password, &payload.password) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with password validation"),
        });
    }

    // Create new token
    let unhashed_token = gen_uuid().to_string();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_token = argon2
        .hash_password(&unhashed_token.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Save new token
    sqlx::query("UPDATE mods SET api_token = $1 WHERE id = $2 RETURNING api_token")
        .bind(hashed_token)
        .bind(user.id)
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
    // Get user
    let user = sqlx::query_as::<_, Mod>("SELECT * FROM mods WHERE org_id = $1 AND name = $2 LIMIT 1")
        .bind(&payload.org_id)
        .bind(payload.name)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    // Auth user
    if verify_hash(&user.password, &payload.password) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    // Get and return org's stream status
    let stream = sqlx::query_as::<_, Org>(r#"SELECT * FROM orgs WHERE org_id = $1"#)
        .bind(user.org_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    Json(AppRes {
        body: Some(stream.status),
        error: None,
    })
}

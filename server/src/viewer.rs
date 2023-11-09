use crate::{db::Mod, db::Org, verify_hash, AppReq, AppRes, AppState};
use axum::{extract, extract::State, Json};
use chrono::{DateTime, Utc};
use rustis::{commands::ListCommands, commands::StringCommands};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Streams {
    names: Vec<StreamNames>,
}

#[derive(Deserialize, Debug)]
pub struct Viewer {
    user_id: String,
    input: String,
    stream: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct StreamNames {
    pub name: String,
}

pub async fn shift(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, Vec<String>>> {
    // Get user and org
    let streamer = sqlx::query_as::<_, Mod>(r#"SELECT * FROM mods WHERE org_id = $1 AND name = $2 LIMIT 1"#)
        .bind(payload.org_id)
        .bind(payload.name)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    let stream =
        sqlx::query_as::<_, Org>(r#"SELECT * FROM orgs WHERE org_id = $1 LIMIT 1"#)
            .bind(&streamer.org_id)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    // Auth user
    if verify_hash(&streamer.api_token, &payload.api_token) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    if stream.status == false {
        return Json(AppRes {
            body: None,
            error: Some("Stream not active"),
        });
    }

    // Get and return 10 inputs
    let res: Vec<String> = state.redis_client.rpop(stream.name, 10).await.unwrap();
    Json(AppRes {
        body: Some(res),
        error: None,
    })
}

#[axum_macros::debug_handler]
pub async fn push(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<Viewer>,
) -> Json<AppRes<'static, &'static str>> {
    // Check if enough time has elapsed for user
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

    // Update time since last message
    let _ = state
        .redis_client
        .set(payload.user_id, Utc::now().to_string())
        .await
        .unwrap();

    // Get org
    let streamer = sqlx::query_as::<_, Org>("SELECT * FROM orgs WHERE name = $1 LIMIT 1")
        .bind(&payload.stream)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    if streamer.status == false {
        return Json(AppRes {
            body: None,
            error: Some("Stream not active"),
        });
    }

    // Save input
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

pub async fn get_active_streamers(State(state): State<AppState>) -> extract::Json<Streams> {
    Json(Streams {
        names: sqlx::query_as("SELECT name FROM orgs WHERE status = True")
            .fetch_all(&state.db_pool)
            .await
            .unwrap(),
    })
}

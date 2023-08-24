use crate::{verify_hash, AppReq, AppRes, AppState, Stream, Streamer, Streams, Viewer};
use axum::{extract, extract::State, Json};
use chrono::{DateTime, Utc};
use rustis::{commands::ListCommands, commands::StringCommands};

pub async fn shift(
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
pub async fn push(
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

pub async fn get_active_streamers(State(state): State<AppState>) -> extract::Json<Streams> {
    Json(Streams {
        names: sqlx::query_as::<_, Stream>(
            "SELECT stream, org_id FROM streamers where active_stream = True",
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap(),
    })
}

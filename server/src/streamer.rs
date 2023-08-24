use crate::{verify_hash, AppReq, AppRes, AppState, Streamer};
use axum::{extract, extract::State, Json};
use rustis::commands::GenericCommands;

pub async fn toggle_stream(
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

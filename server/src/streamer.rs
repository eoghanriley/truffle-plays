use crate::{db::Mod, db::Stream, verify_hash, AppReq, AppRes, AppState};
use axum::{extract, extract::State, Json};
use rustis::commands::GenericCommands;

pub async fn toggle_stream(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, bool>> {
    let user = sqlx::query_as::<_, Mod>(r#"SELECT * FROM mods WHERE id = $1 LIMIT 1"#)
        .bind(&payload.id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    let mut stream =
        sqlx::query_as::<_, Stream>(r#"SELECT status, org_id FROM streams WHERE name = $1"#)
            .bind(&user.stream)
            .fetch_one(&state.db_pool)
            .await
            .unwrap();

    if verify_hash(&user.api_token.unwrap(), &payload.api_token) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    stream.status = Some(!stream.status.unwrap());

    sqlx::query(r#"UPDATE streams SET status = $1 WHERE org_id = $2 RETURNING active_stream"#)
        .bind(&stream.status.unwrap())
        .bind(stream.org_id.unwrap())
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    if &stream.status.unwrap() == &false {
        let _ = state.redis_client.del(stream.name);
    }

    Json(AppRes {
        body: Some(stream.status.unwrap()),
        error: None,
    })
}

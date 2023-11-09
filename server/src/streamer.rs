use crate::{db::Mod, db::Org, verify_hash, AppReq, AppRes, AppState};
use axum::{extract, extract::State, Json};
use rustis::commands::GenericCommands;

pub async fn toggle_stream(
    State(mut state): State<AppState>,
    extract::Json(payload): extract::Json<AppReq>,
) -> Json<AppRes<'static, bool>> {
    // Get user and org
    let user = sqlx::query_as::<_, Mod>(r#"SELECT * FROM mods WHERE org_id = $1 AND name = $2 LIMIT 1"#)
        .bind(&payload.org_id)
        .bind(&payload.name)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    let mut stream = sqlx::query_as::<_, Org>(r#"SELECT * FROM orgs WHERE org_id = $1"#)
        .bind(&user.org_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    if verify_hash(&user.api_token, &payload.api_token) == false {
        return Json(AppRes {
            body: None,
            error: Some("Error with validation"),
        });
    }

    // Update orgs stream status
    stream.status = !stream.status;

    sqlx::query(r#"UPDATE orgs SET status = $1 WHERE org_id = $2 RETURNING status"#)
        .bind(&stream.status)
        .bind(user.org_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    // Remove inputs waiting to be processed
    if &stream.status == &false {
        let _ = state.redis_client.del(stream.name);
    }

    Json(AppRes {
        body: Some(stream.status),
        error: None,
    })
}

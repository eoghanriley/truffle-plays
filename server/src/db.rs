use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Streamer {
    pub id: Option<i32>,
    pub org_id: String,
    pub password: String,
    pub stream: String,
    pub api_token: Option<String>,
    pub donater: Option<bool>,
    pub active_stream: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct RegisterLinks {
    pub link: String,
    pub used: bool,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Stream {
    pub stream: String,
    pub org_id: String,
}

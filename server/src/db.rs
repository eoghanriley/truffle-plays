use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Stream {
    pub id: Option<i32>,
    pub org_id: String,
    pub name: String,
    pub active_stream: bool,
    pub mods: Vec<String>, // Mod ids
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub password: String,
    pub api_token: String,
    pub root: bool,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct RegisterLinks {
    pub link: String,
    pub used: bool,
}

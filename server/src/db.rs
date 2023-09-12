use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Org {
    pub id: i32,
    pub org_id: String,
    pub name: String,
    pub status: bool,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub password: String,
    pub org_id: String,
    pub api_token: String,
    pub root: bool,
    pub receiver: bool,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct RegisterLinks {
    pub link: String,
    pub used: bool,
}

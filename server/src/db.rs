use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Org {
    pub id: Option<i32>,
    pub org_id: Option<String>,
    pub name: Option<String>,
    pub status: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Mod {
    pub id: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub org_id: Option<String>,
    pub api_token: Option<String>,
    pub root: Option<bool>,
    pub reciever: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct RegisterLinks {
    pub link: String,
    pub used: bool,
}

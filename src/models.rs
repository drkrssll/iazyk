use crate::schema::players;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = players)]
pub struct Player {
    pub id: i32,
    pub rank: String,
    pub kills: i64,
    pub headshots: i64,
    pub bank: i64,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = players)]
pub struct NewPlayer {
    pub id: i32,
    pub rank: String,
    pub kills: i64,
    pub headshots: i64,
    pub bank: i64,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = players)]
pub struct UpdatePlayer {
    pub rank: Option<String>,
    pub kills: Option<i64>,
    pub headshots: Option<i64>,
    pub bank: Option<i64>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

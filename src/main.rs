#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use std::collections::HashMap;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

mod db;
mod models;
mod schema;

use db::DbPool;
use models::{ErrorResponse, NewPlayer, Player, UpdatePlayer};
use schema::players;

#[get("/players/<player_id>/bank")]
fn get_player_bank(pool: &State<DbPool>, player_id: i32) -> Option<String> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let bank = players::table
        .find(player_id)
        .select(players::bank)
        .first::<i64>(connection)
        .optional()
        .expect("Error loading player bank");

    bank.map(|b| b.to_string())
}

#[get("/players/<player_id>/rank")]
fn get_player_rank(pool: &State<DbPool>, player_id: i32) -> Option<String> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let rank = players::table
        .find(player_id)
        .select(players::rank)
        .first::<String>(connection)
        .optional()
        .expect("Error loading player rank");

    rank.map(|b| b.to_string())
}

#[get("/players/<player_id>")]
fn get_player(pool: &State<DbPool>, player_id: i32) -> Result<Json<Player>, Json<ErrorResponse>> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let result = players::table
        .find(player_id)
        .first::<Player>(connection)
        .optional()
        .expect("Error loading player");

    result.map(Json).ok_or_else(|| {
        Json(ErrorResponse {
            error: "Player not found".to_string(),
        })
    })
}

#[post("/players/<player_id>/lvlup", data = "<new_rank>")]
fn level_up_player(
    pool: &State<DbPool>,
    player_id: i32,
    new_rank: Json<String>,
) -> Option<Json<Player>> {
    let ranks = vec![
        "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "B0", "B1", "B2", "B3", "B4",
        "B5", "B6", "B7", "B8", "B9", "C0", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9",
    ];

    if !ranks.contains(&new_rank.as_str()) {
        return None;
    }

    let connection = &mut pool.get().expect("Failed to get connection");
    let updated_rank = diesel::update(players::table.find(player_id))
        .set(players::rank.eq(&*new_rank))
        .get_result::<Player>(connection)
        .optional()
        .expect("Error leveling up player");

    updated_rank.map(Json)
}

#[post("/players", data = "<new_player>")]
fn create_player(pool: &State<DbPool>, new_player: Json<NewPlayer>) -> Json<Player> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let inserted_player = diesel::insert_into(players::table)
        .values(&*new_player)
        .get_result::<Player>(connection)
        .expect("Error saving new player");

    Json(inserted_player)
}

#[post("/players/<player_id>", data = "<update_player>")]
fn update_player(
    pool: &State<DbPool>,
    player_id: i32,
    update_player: Json<UpdatePlayer>,
) -> Option<Json<Player>> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let updated_player = diesel::update(players::table.find(player_id))
        .set(&*update_player)
        .get_result::<Player>(connection)
        .optional()
        .expect("Error updating player");
    updated_player.map(Json)
}

#[delete("/players/<player_id>")]
fn delete_player(pool: &State<DbPool>, player_id: i32) -> Option<()> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let num_deleted = diesel::delete(players::table.find(player_id))
        .execute(connection)
        .expect("Error deleting player");

    if num_deleted > 0 {
        Some(())
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    let pool = db::init_pool();

    rocket::build().manage(pool).mount(
        "/",
        routes![
            get_player,
            get_player_bank,
            get_player_rank,
            level_up_player,
            create_player,
            update_player,
            delete_player
        ],
    )
}

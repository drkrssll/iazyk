#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

mod db;
mod models;
mod schema;

use db::DbPool;
use models::{ErrorResponse, NewPlayer, Player, UpdatePlayer};
use schema::players;
use serde::Deserialize;

#[derive(Deserialize)]
struct RankUpdate {
    rank: String,
    bank: i64,
}

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

#[get("/players/<player_id>/kills")]
fn get_player_kills(pool: &State<DbPool>, player_id: i32) -> Option<String> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let bank = players::table
        .find(player_id)
        .select(players::kills)
        .first::<i64>(connection)
        .optional()
        .expect("Error loading player bank");

    bank.map(|b| b.to_string())
}

#[get("/players/<player_id>/headshots")]
fn get_player_headshots(pool: &State<DbPool>, player_id: i32) -> Option<String> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let bank = players::table
        .find(player_id)
        .select(players::headshots)
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

#[get("/players/<player_id>/lvlup")]
fn level_up_player(pool: &State<DbPool>, player_id: i32) -> Option<String> {
    let ranks = vec![
        "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "B0", "B1", "B2", "B3", "B4", "B5",
        "B6", "B7", "B8", "B9", "C0", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9",
    ];

    let requirements = vec![
        (500, 50),
        (800, 100),
        (1000, 150),
        (1400, 200),
        (1800, 250),
        (2200, 300),
        (2600, 350),
        (3000, 400),
        (3500, 450),
        (4000, 500),
        (4500, 550),
        (5000, 600),
        (5500, 650),
        (6000, 700),
        (6500, 750),
        (7000, 800),
        (7500, 850),
        (8000, 900),
        (8500, 950),
        (9000, 1000),
        (9500, 1050),
        (10000, 1100),
        (10500, 1150),
        (11000, 1200),
        (11500, 1250),
        (12000, 1300),
        (12500, 1350),
        (13000, 1400),
        (13500, 1450),
    ];

    let connection = &mut pool.get().expect("Failed to get connection");

    let current_rank = players::table
        .find(player_id)
        .select(players::rank)
        .first::<String>(connection)
        .optional()
        .expect("Error loading player rank");

    let kills = players::table
        .find(player_id)
        .select(players::kills)
        .first::<i64>(connection)
        .optional()
        .expect("Error loading player kills");

    let headshots = players::table
        .find(player_id)
        .select(players::headshots)
        .first::<i64>(connection)
        .optional()
        .expect("Error loading player headshots");

    if current_rank.as_deref().unwrap() == "C9" {
        return current_rank.map(|r| r.to_string());
    } else if current_rank.as_deref().unwrap() == "A0" {
        if kills.unwrap() >= requirements[0].0 as i64
            && headshots.unwrap() >= requirements[0].1 as i64
        {
            diesel::update(players::table.find(player_id))
                .set(players::rank.eq("A1"))
                .execute(connection)
                .expect("Error updating player rank");
        }
    } else if let Some(index) = ranks
        .iter()
        .position(|&r| r == current_rank.as_deref().unwrap())
    {
        let next_index = index + 1;
        if next_index < ranks.len() {
            let (req_kills, req_headshots) = requirements[next_index];
            if kills.unwrap() >= req_kills as i64 && headshots.unwrap() >= req_headshots as i64 {
                diesel::update(players::table.find(player_id))
                    .set(players::rank.eq(ranks[next_index]))
                    .execute(connection)
                    .expect("Error updating player rank");
            }
        }
    }

    let final_rank = players::table
        .find(player_id)
        .select(players::rank)
        .first::<String>(connection)
        .optional()
        .expect("Error loading player rank");

    final_rank.map(|r| r.to_string())
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
            get_player_kills,
            get_player_headshots,
            get_player_rank,
            level_up_player,
            create_player,
            update_player,
            delete_player
        ],
    )
}

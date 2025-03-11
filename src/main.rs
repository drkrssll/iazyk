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
use models::{NewPlayer, Player, UpdatePlayer};
use schema::players;

#[get("/players")]
fn get_players(pool: &State<DbPool>) -> Json<Vec<Player>> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let results = players::table
        .load::<Player>(connection)
        .expect("Error loading players");
    Json(results)
}

#[get("/players/<player_id>")]
fn get_player(pool: &State<DbPool>, player_id: i32) -> Option<Json<Player>> {
    let connection = &mut pool.get().expect("Failed to get connection");
    let result = players::table
        .find(player_id)
        .first::<Player>(connection)
        .optional()
        .expect("Error loading player");
    result.map(Json)
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

#[put("/players/<player_id>", data = "<update_player>")]
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
            get_players,
            get_player,
            create_player,
            update_player,
            delete_player
        ],
    )
}

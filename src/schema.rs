// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int4,
        rank -> Varchar,
        kills -> Int8,
        headshots -> Int8,
        bank -> Int8,
    }
}

-- Your SQL goes here
CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    rank VARCHAR NOT NULL,
    kills BIGINT NOT NULL DEFAULT 0,
    headshots BIGINT NOT NULL DEFAULT 0,
    bank BIGINT NOT NULL DEFAULT 0
    multiplier BIGINT NOT NULL DEFAULT 1,
)

-- Your SQL goes here

CREATE TABLE Person (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    sex SMALLINT NOT NULL
)

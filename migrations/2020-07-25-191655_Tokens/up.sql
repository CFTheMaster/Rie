-- Your SQL goes here
CREATE TABLE Tokens (
    id SERIAL PRIMARY KEY NOT NULL,
    token_type VARCHAR NOT NULL,
    token VARCHAR NOT NULL
)
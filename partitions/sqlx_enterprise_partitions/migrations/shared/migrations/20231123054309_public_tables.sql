-- Add migration script here
CREATE TABLE fellow_kenyans (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    other_names TEXT,
    birth_date TIMESTAMP NOT NULL
);

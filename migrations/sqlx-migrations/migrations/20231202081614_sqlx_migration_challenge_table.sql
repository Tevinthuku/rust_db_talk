-- Add migration script here
CREATE TABLE IF NOT EXISTS sqlx_migration_challenge (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255)
);

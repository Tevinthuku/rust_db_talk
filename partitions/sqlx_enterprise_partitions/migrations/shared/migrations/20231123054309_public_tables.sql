-- Add migration script here
CREATE TABLE fellow_kenyans (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    other_names TEXT,
    birth_date TIMESTAMP NOT NULL
);

-- populate table with random data

INSERT INTO fellow_kenyans (first_name, last_name, other_names, birth_date)
SELECT
    'First' || id AS first_name,
    'Last' || id AS last_name,
    'Other' || id AS other_names,
    CURRENT_DATE - INTERVAL '1' DAY * (id % 365) AS birth_date
FROM
    generate_series(1, 55000000) AS id;

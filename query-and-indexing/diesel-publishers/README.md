## Info

Just a basic system that stores author details and the books they wrote.

We cover querying the database with diesel, indexes and tracing

## Setup

1.  run `diesel setup`
2.  run `diesel migration run`
3.  populate db with seed data from `seed/dummy_data.sql`

## Running the commands

### Author details

```
cargo run --bin=diesel-publishers details 1000
```

### Author books with diesel dsl

```
cargo run --bin=diesel-publishers books-diesel-dsl 10000
```

### Author books with raw sql

```
cargo run --bin=diesel-publishers books-raw-sql 1000
```

### Inspecting the DB

```
docker exec -it rust_db_talk-base_db-1 psql -U postgres -d diesel_publishers_db
```
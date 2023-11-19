## Info

Just a basic system that stores author details and the books they wrote.

We cover querying the database with diesel, indexes and tracing

## Setup

1.  run `diesel setup`
2.  run `diesel migration run`

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

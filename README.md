## What you need to install

#### Sqlx

https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#sqlx-cli

if you only want postgres support.

```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

#### Diesel CLI

https://diesel.rs/guides/getting-started

if you only want postgres support.

```
cargo install diesel_cli --no-default-features --features postgres
```

#### Docker desktop

https://www.docker.com/products/docker-desktop/

#### docker-compose up command

```
docker compose -f docker-compose.yml up -d
```

## Session structure

We'll cover the topics in the following order

1. connection pooling. -> `connection-pool` crate.  
   -> Has Both SQLX and Diesel connection pools and fixtures (for tests)

2. migrations. -> `migrations` folder. <br />
   `migrations/diesel-migrations` -> How to handle diesel migrations. (Challenge included) <br />
   `migrations/sqlx-migrations` -> SQLX migrations. (Challenge included)

3. querying, indexing and tracing. -> `query-and-indexing/diesel-publishers` crate. <br /> We'll use a book publishing example to showcase the 3 actions. (Including tests)

4. transactions -> `SQLX` -> We'll look at an "online" store example. We'll look at product creation and report generation.

5. partitions -> `partitions` folder (Challenges included in both) <br />
   `partitions/diesel_pg_table_partitions` -> Postgres table partitions using Diesel ORM <br />
   `partitions/sqlx_enterprise_partitions` -> A multitenant setup in postgres with SQLX



## Accessing the DB via the UI:

Download https://tableplus.com/download

Download db connection group from here https://drive.google.com/drive/folders/1BBABdXIGb2O5jufwh56V_nTp1cBYSclA?usp=sharing


## Accessing the DB via the terminal:


```
docker exec -it <container_name> psql -U postgres -d <db_name>
```

eg: The base_db
```
docker exec -it rust_db_talk-base_db-1  psql -U postgres -d base_db
```

eg: pg_table_partition
```
docker exec -it rust_db_talk-pg_table_partitions-1  psql -U postgres -d pg_table_partitions
```


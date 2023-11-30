## What you need to install

#### Sqlx

https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#sqlx-cli

#### Diesel CLI

https://diesel.rs/guides/getting-started

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

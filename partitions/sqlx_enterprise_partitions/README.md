## SQLX goes enterprise

A Multi-tenant sqlx setup.

### Setup

```
sqlx database create
```

#### Running the migrations

```
1.  cargo run -- run-migrations
```

```
2.  populate db with seed data from `seed/insert_55_million_kenyans.sql``
```

#### Visiting a hospital

```
2. cargo run -- visit-hospital <hospital_name> <kenyan_id>
```

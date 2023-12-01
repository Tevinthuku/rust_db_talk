https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md


Setup

```
sqlx database create
```

Example message we get when you create your first sqlx migration:

```

➜  sqlx_migrations git:(main) ✗ sqlx migrate add test_table
Creating migrations/20231115041202_test_table.sql

Congratulations on creating your first migration!

Did you know you can embed your migrations in your application binary?
On startup, after creating your database connection or pool, add:

sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.

```


Running the migration

```
sqlx migrate run
```

use anyhow::Context;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use dotenv::dotenv;

    dotenv().ok();

    let _sqlx_pool = create_sqlx_pool().await?;
    let _diesel_pool = create_diesel_pool()?;

    Ok(())
}

async fn create_sqlx_pool() -> anyhow::Result<PgPool> {
    use sqlx::postgres::PgPoolOptions;
    let db_url =
        std::env::var("DATABASE_URL").context("Failed to get the DATABASE_URL env variable")?;
    PgPoolOptions::new()
        .max_connections(20)
        // there's many connect configurations; eg: `connect_lazy`
        .connect(&db_url)
        .await
        .context("Failed to create a pool")
}

type DieselPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

fn create_diesel_pool() -> anyhow::Result<DieselPool> {
    let db_url =
        std::env::var("DATABASE_URL").context("Failed to get the DATABASE_URL env variable")?;
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    diesel::r2d2::Builder::new()
        .max_size(20)
        // there's also `build_unchecked` that does not establish any connections initially.
        .build(manager)
        .context("Failed to build the diesel pool")
}

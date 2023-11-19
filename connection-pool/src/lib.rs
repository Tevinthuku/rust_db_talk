use anyhow::Context;
use diesel::r2d2::ConnectionManager;
use sqlx::PgPool;

pub async fn create_sqlx_pool() -> anyhow::Result<PgPool> {
    use sqlx::postgres::PgPoolOptions;
    let db_url = read_db_url_from_env()?;
    PgPoolOptions::new()
        .max_connections(20)
        // there's many connect configurations; eg: `connect_lazy`
        .connect(&db_url)
        .await
        .context("Failed to create a pool")
}

pub type DieselPool = diesel::r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

pub fn create_diesel_pool() -> anyhow::Result<DieselPool> {
    let db_url = read_db_url_from_env()?;
    let manager = ConnectionManager::<diesel::PgConnection>::new(db_url);
    diesel::r2d2::Builder::new()
        .max_size(20)
        // there's also `build_unchecked` that does not establish any connections initially.
        .build(manager)
        .context("Failed to build the diesel pool")
}

fn read_db_url_from_env() -> anyhow::Result<String> {
    dotenv_flow::dotenv_flow().ok();

    std::env::var("DATABASE_URL").context("Failed to get the DATABASE_URL env variable")
}

use anyhow::Context;
use connection_pool::create_sqlx_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_sqlx_pool().await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Fauled to run migration")
}

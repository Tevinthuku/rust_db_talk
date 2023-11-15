use connection_pool::{create_diesel_pool, create_sqlx_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _sqlx_pool = create_sqlx_pool().await?;
    let _diesel_pool = create_diesel_pool()?;

    Ok(())
}

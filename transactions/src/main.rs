pub mod create_product;
pub mod generate_report;

use anyhow::Context;
use clap::Parser;
use connection_pool::create_sqlx_pool;
use sqlx::PgPool;
use tracing_log::log::{error, info};

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    query: Query,
}

#[derive(Debug, Parser, Clone)]
enum Query {
    CreateProduct,
    GenerateReport,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv_flow::dotenv_flow().ok();
    let pool = create_sqlx_pool().await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing_helpers::config_telemetry();

    let cli = Options::parse();
    if let Err(err) = start(pool, cli.query).await {
        error!("{err:?}");
    }

    tracing_helpers::shutdown_global_tracer_provider();

    Ok(())
}

#[tracing::instrument(skip(pool), level = "info")]
async fn start(pool: PgPool, query: Query) -> anyhow::Result<()> {
    match query {
        Query::CreateProduct => create_product::create_product(pool).await.map(|sku| {
            info!("The product_sku is {sku}");
        }),
        Query::GenerateReport => generate_report::generate_report(pool).await,
    }
}

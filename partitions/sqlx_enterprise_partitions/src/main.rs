pub mod migration;
pub mod tenant;
#[cfg(test)]
pub mod test_fixtures;

pub mod visit_hospital;

use clap::Parser;
use connection_pool::create_sqlx_pool;
use sqlx::PgPool;
use std::sync::Arc;
use tenant::TenantConnection;
use tracing::error;
use visit_hospital::visit_hospital;

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    query: Query,
}

#[derive(Debug, Parser, Clone)]
enum Query {
    RunMigrations,
    VisitHospital { hospital: String, id: i32 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv_flow::dotenv_flow().ok();
    let pool = create_sqlx_pool().await.map(Arc::new)?;
    let cli = Options::parse();
    tracing_helpers::config_telemetry();

    if let Err(err) = start(pool, cli.query).await {
        error!("{err:?}");
    }
    tracing_helpers::shutdown_global_tracer_provider();

    Ok(())
}

#[tracing::instrument(skip(pool), level = "info")]
async fn start(pool: Arc<PgPool>, query: Query) -> anyhow::Result<()> {
    match query {
        Query::RunMigrations => migration::run_migrations(&pool).await,
        Query::VisitHospital { id, hospital } => {
            let tenant_conn = TenantConnection::new(pool, hospital)?;
            visit_hospital(&tenant_conn, id).await
        }
    }
}

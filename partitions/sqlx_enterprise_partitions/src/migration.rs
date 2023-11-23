use anyhow::Context;
use sqlx::{migrate::Migrator, PgPool};

static SHARED_MIGRATOR: Migrator = sqlx::migrate!("./migrations/shared");
static TENANT_MIGRATOR: Migrator = sqlx::migrate!("./migrations/tenant");

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin a transaction")?;
    SHARED_MIGRATOR
        .run(&mut *tx)
        .await
        .context("Failed to run the shared migrations")?;
    let tenants = ["nairobi-west", "mater"];
    for tenant in tenants {
        let create_schema_query = format!("CREATE SCHEMA IF NOT EXISTS \"{}\"", tenant);
        sqlx::query(&create_schema_query)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("Failed to create schema for tenant {tenant}"))?;

        let set_search_path_query = format!("SET search_path TO \"{}\"", tenant);
        sqlx::query(&set_search_path_query)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("Failed to set tenant search path for tenant {tenant}"))?;

        TENANT_MIGRATOR
            .run(&mut *tx)
            .await
            .with_context(|| format!("Failed to run the migrations for tenant {tenant}"))?;
    }
    Ok(())
}

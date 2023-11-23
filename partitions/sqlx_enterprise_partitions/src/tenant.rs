use anyhow::{bail, Context};
use sqlx::{pool::PoolConnection, PgConnection, PgPool, Postgres};

pub static TENANTS: [&str; 2] = ["mater", "nairobi-west"];

pub struct TenantConnection<'a> {
    pool: &'a PgPool,
    tenant: String,
}

impl<'a> TenantConnection<'a> {
    pub fn new(pool: &'a PgPool, tenant: String) -> anyhow::Result<Self> {
        if tenant.contains(&tenant) {
            return Ok(Self { pool, tenant });
        }
        bail!("No tenant found with name {tenant}")
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn conn(&self) -> anyhow::Result<PoolConnection<Postgres>> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .with_context(|| format!("Failed to get a connection for {}", &self.tenant))?;
        self.set_search_path(&mut conn).await?;
        Ok(conn)
    }

    #[tracing::instrument(skip(self, conn), level = "info")]
    async fn set_search_path(&self, conn: &mut PgConnection) -> anyhow::Result<()> {
        let search_path_query = format!("SET search_path=\"{}\",public", &self.tenant);

        sqlx::query(&search_path_query)
            .execute(conn)
            .await
            .map(|_| ())
            .with_context(|| format!("Failed to set_search_path for tenant {}", self.tenant))
    }
}

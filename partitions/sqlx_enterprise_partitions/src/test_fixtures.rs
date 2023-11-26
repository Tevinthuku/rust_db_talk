use connection_pool::test_fixtures::sqlx_pool_fixture;
use rstest::*;
use sqlx::PgPool;
use std::sync::Arc;

use crate::tenant::{TenantConnection, TENANTS};

#[fixture]
pub async fn test_hospital_tenant_connection(
    #[future] sqlx_pool_fixture: PgPool,
) -> TenantConnection {
    let pool = Arc::new(sqlx_pool_fixture.await);

    crate::migration::run_migrations(&pool).await.unwrap();

    sqlx::query(
        "INSERT INTO fellow_kenyans (first_name, last_name, birth_date) VALUES ('John', 'Doe', '2020-01-01')",
    ).execute(&*pool).await.unwrap();

    TenantConnection::new(pool, TENANTS[0].to_owned()).unwrap()
}

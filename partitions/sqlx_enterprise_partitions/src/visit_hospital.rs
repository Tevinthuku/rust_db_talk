use anyhow::Context;
use tracing::info;

use crate::tenant::TenantConnection;

#[tracing::instrument(skip(pool), level = "info")]
pub async fn visit_hospital(pool: &TenantConnection, id: i32) -> anyhow::Result<()> {
    let mut conn = pool.conn().await?;
    #[derive(sqlx::FromRow)]
    struct HospitalVisit {
        id: i32,
    }

    sqlx::query_as::<_, HospitalVisit>(
        "
        INSERT INTO hospital_visits (patient_id) VALUES ($1) RETURNING id
    ",
    )
    .bind(id)
    .fetch_one(&mut *conn)
    .await
    .map(|visit| {
        let HospitalVisit { id } = visit;
        info!("The hospital visit id is {id}");
    })
    .context("Failed to insert hospital_visit")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tenant::TenantConnection;
    use rstest::*;
    use crate::test_fixtures::test_hospital_tenant_connection;

    #[rstest]
    #[tokio::test]
    async fn test_creating_a_hospital_visit_works(
        #[future] test_hospital_tenant_connection: TenantConnection,
    ) {
        let conn = test_hospital_tenant_connection.await;
        let result = visit_hospital(&conn, 1).await;
        assert!(result.is_ok())
    }
}

use anyhow::Context;
use tracing::info;

use crate::tenant::TenantConnection;

#[tracing::instrument(skip(pool), level = "info")]
pub async fn visit_hospital(pool: &TenantConnection<'_>, id: i32) -> anyhow::Result<()> {
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

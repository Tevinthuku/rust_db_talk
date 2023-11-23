use anyhow::Context;
use sqlx::Row;
use tracing::info;

use crate::tenant::TenantConnection;

#[tracing::instrument(skip(pool), level = "info")]
pub async fn visit_hospital(pool: &TenantConnection<'_>, id: i32) -> anyhow::Result<()> {
    let mut conn = pool.conn().await?;

    sqlx::query(
        "
        INSERT INTO hospital_visits (patient_id) VALUES ($1) RETURNING id
    ",
    )
    .bind(id)
    .fetch_one(&mut *conn)
    .await
    .map(|row| {
        // Note that this will panic if the "id" field does not exist.
        let id: i32 = row.get("id");
        info!("The patient visit id is {id}")
    })
    .context("Failed to insert hospital_visit")
}

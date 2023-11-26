use anyhow::{anyhow, Context};
use sqlx::{Executor, PgPool};

#[tracing::instrument(skip(pool), level = "info")]
pub async fn generate_report(pool: PgPool) -> anyhow::Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;
    // https://github.com/launchbadge/sqlx/issues/481
    // we still don't have "native" sqlx support for setting the isolation level
    // Diesel, does support this https://docs.diesel.rs/2.0.x/diesel/pg/struct.TransactionBuilder.html#method.repeatable_read
    tx.execute("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;")
        .await
        .context("Failed to set isolation level")?;

    let summary = sqlx::query!(
        "
            SELECT product_sku, SUM(quantity) as total_quantity, SUM(price) as total_sales FROM product_sale GROUP BY (product_sku)
        "
    )
    .fetch_all(&mut *tx).await.context("Failed to get sales summary")?;

    println!("\n === SALES SUMMARY === \n");
    for summary_item in summary {
        let product_sku = summary_item.product_sku;
        let total_quantity = summary_item.total_quantity.ok_or_else(|| {
            anyhow!(
                "Failed to get the total_quantity sold for {} ",
                &product_sku
            )
        })?;
        let total_sum = summary_item.total_sales.ok_or_else(|| {
            anyhow!(
                "Failed to get the total_quantity sold for {} ",
                &product_sku
            )
        })?;

        println!("SKU {product_sku}, TOTAL QUANTITY {total_quantity}, TOTAL SUM {total_sum}")
    }

    let individual_sales = sqlx::query!(
        "
            SELECT sale_date, product_sku, quantity, price FROM product_sale ORDER BY sale_date;
        "
    )
    .fetch_all(&mut *tx)
    .await
    .context("Failed to get indidual sales")?;

    println!("\n === INDIVIDUAL SALES === \n");
    for sale in individual_sales {
        let sale_date = sale.sale_date;
        let product_sku = sale.product_sku;
        let quantity = sale.quantity;
        let price = sale.price;

        println!("SALE DATE {sale_date}, SKU {product_sku}, QUANTITY {quantity}, PRICE {price}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::generate_report::*;

    #[sqlx::test]
    async fn generate_report_works(pool: PgPool) -> sqlx::Result<()> {
        let result = generate_report(pool).await;
        assert!(result.is_ok());
        Ok(())
    }
}

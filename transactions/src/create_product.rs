use anyhow::Context;
use bigdecimal::BigDecimal;

use fake::{Fake, Faker};
use sqlx::PgPool;

#[tracing::instrument(skip(pool), level = "debug")]
pub async fn create_product(pool: PgPool) -> anyhow::Result<String> {
    let mut tx = pool.begin().await?;
    let product_sku: String = Faker.fake();
    let product_name: String = Faker.fake();
    let product_price = BigDecimal::from(Faker.fake::<i8>());
    let data = sqlx::query!(
        r#"
        INSERT INTO product (product_sku, name, price)

        VALUES ($1, $2, $3) 
        RETURNING product_sku
    "#,
        product_sku,
        product_name,
        product_price
    )
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert the product details")?;

    // trying to ensure that the random quantity is always a positive i32
    let product_quantity = Faker.fake::<i32>().abs();
    sqlx::query!(
        r#"
        INSERT INTO product_inventory (product_sku, quantity) VALUES ($1, $2) 
        "#,
        &data.product_sku,
        product_quantity
    )
    .execute(&mut *tx)
    .await
    .context("Failed to insert the inventory")?;

    tx.commit().await.context("Failed to commit changes")?;

    Ok(data.product_sku)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::create_product::*;

    #[sqlx::test]
    async fn create_test_works(pool: PgPool) -> sqlx::Result<()> {
        let result = create_product(pool).await;
        println!("{result:?}");
        assert!(result.is_ok());

        Ok(())
    }
}

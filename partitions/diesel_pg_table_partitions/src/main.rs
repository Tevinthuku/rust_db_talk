pub mod schema;

use std::fmt::Debug;

use anyhow::Context;
use diesel::{ExpressionMethods, RunQueryDsl};

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use clap::Parser;
use connection_pool::{create_diesel_pool, DieselPool};
use diesel::{connection::SimpleConnection, deserialize::Queryable};
use tracing::{error, info};

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    query: Query,
}

#[derive(Debug, Parser, Clone)]
enum Query {
    CreatePartition {
        name: String,
        from: String,
        to: String,
    },
    NewSale {
        product_id: i32,
        sale_date: String,
        amount: u8,
    },
    ReadSales {
        from: String,
        to: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_helpers::config_telemetry();
    let pool = create_diesel_pool()?;
    if let Err(err) = start(pool).await {
        error!("{err:?}");
    }
    tracing_helpers::shutdown_global_tracer_provider();

    Ok(())
}

#[tracing::instrument(level = "info", skip(pool))]
async fn start(pool: DieselPool) -> anyhow::Result<()> {
    dotenv_flow::dotenv_flow().ok();
    let cli = Options::parse();
    query_runner(pool, cli.query).await
}

#[tracing::instrument(skip(pool))]
async fn query_runner(pool: DieselPool, query: Query) -> anyhow::Result<()> {
    match query {
        Query::CreatePartition { name, from, to } => create_partition(pool, name, from, to).await,
        Query::NewSale {
            product_id,
            sale_date,
            amount,
        } => new_sale(pool, product_id, sale_date, amount).await,
        Query::ReadSales { from, to } => read_sales(pool, from, to).await,
    }
}

#[tracing::instrument(skip(pool))]
async fn create_partition(
    pool: DieselPool,
    name: String,
    from: String,
    to: String,
) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Failed to get a connection")?;
        let statement = format!(
            "
        CREATE TABLE {name} PARTITION OF sales
        FOR VALUES FROM ('{from}') TO ('{to}');
        "
        );
        conn.batch_execute(&statement)
            .context("Failed to create the partition")
    })
    .await?
}

#[tracing::instrument(skip(pool))]
async fn new_sale(
    pool: DieselPool,
    product_id: i32,
    sale_date: String,
    amount: u8,
) -> anyhow::Result<()> {
    use schema::sales;
    let amount = BigDecimal::from(amount);
    let sale_date =
        NaiveDate::parse_from_str(&sale_date, DATE_FORMAT).context("Failed to parse date")?;
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Failed to get a connection")?;
        diesel::insert_into(sales::table)
            .values((
                sales::product_id.eq(product_id),
                sales::sale_date.eq(sale_date),
                sales::amount.eq(amount),
            ))
            .execute(&mut conn)
            .map(|_| ())
            .context("Failed to insert new sale")
    })
    .await?
}

#[derive(Queryable, Debug)]
#[allow(dead_code)]
struct Sale {
    id: i32,
    product_id: i32,
    sale_date: NaiveDate,
    amount: BigDecimal,
}

#[tracing::instrument(skip(pool))]
async fn read_sales(pool: DieselPool, from: String, to: String) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use schema::sales::dsl::*;

    use diesel::debug_query;
    let from =
        NaiveDate::parse_from_str(&from, DATE_FORMAT).context("Failed to parse from date")?;
    let to = NaiveDate::parse_from_str(&to, DATE_FORMAT).context("Failed to parse to date")?;

    let query = sales.filter(sale_date.between(from, to));
    let query_as_str = debug_query::<diesel::pg::Pg, _>(&query).to_string();
    let results = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Failed to get a connection")?;
        query
            .load::<Sale>(&mut conn)
            .context("Failed to get results")
    })
    .await??;

    info!(the_query = query_as_str, sales = format!("{results:?}"));

    Ok(())
}

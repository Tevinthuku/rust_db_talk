pub mod author;
pub mod schema;

#[cfg(test)]
pub mod fixtures;
use std::fmt;
use std::fmt::Debug;

use clap::Parser;
use connection_pool::{create_diesel_pool, DieselPool};
use tracing::{error, info};

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    query: Query,
}

#[derive(Debug, Parser, Clone, Copy)]
enum Query {
    Details { id: i32 },
    BooksDieselDsl { id: i32 },
    BooksRawSql { id: i32 },
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Query::Details { id } => write!(f, "Author details id =  {id}"),
            Query::BooksDieselDsl { id } => write!(f, "Author books with diesel dsl id = {id}"),
            Query::BooksRawSql { id } => write!(f, "Author books with raw sql id = {id}"),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_helpers::config_telemetry();
    start().await?;
    tracing_helpers::shutdown_global_tracer_provider();

    Ok(())
}

#[tracing::instrument(level = "info")]
async fn start() -> anyhow::Result<()> {
    dotenv_flow::dotenv_flow().ok();

    let cli = Options::parse();

    let pool = create_diesel_pool()?;
    let result = query_runner(pool, cli.query).await;

    match result {
        Ok(res) => info!("{res:?}"),
        Err(err) => error!("{err:?}"),
    }

    Ok(())
}

#[tracing::instrument(skip(pool), level = "info")]
async fn query_runner(pool: DieselPool, query: Query) -> anyhow::Result<Box<dyn Debug>> {
    let db_result: Box<dyn Debug> = match query {
        Query::Details { id } => Box::new(author::author_details(pool, id).await?),
        Query::BooksDieselDsl { id } => Box::new(author::author_books_diesel_dsl(pool, id).await?),
        Query::BooksRawSql { id } => Box::new(author::author_books_raw_sql(pool, id).await?),
    };
    info!(
        query = query.to_string(),
        database_results = format!("{db_result:?}")
    );
    Ok(db_result)
}

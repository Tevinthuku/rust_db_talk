pub mod author;
pub mod schema;
use std::fmt::Debug;

use clap::Parser;
use connection_pool::create_diesel_pool;

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    author: Query,
}

#[derive(Debug, Parser, Clone, Copy)]
enum Query {
    Details { id: i32 },
    BooksDieselDsl { id: i32 },
    BooksRawSql { id: i32 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Options::parse();
    let pool = create_diesel_pool()?;
    let author_query = cli.author;
    let db_result: Box<dyn Debug> = match author_query {
        Query::Details { id } => Box::new(author::author_details(pool, id).await?),
        Query::BooksDieselDsl { id } => Box::new(author::author_books_diesel_dsl(pool, id).await?),
        Query::BooksRawSql { id } => Box::new(author::author_books_raw_sql(pool, id).await?),
    };

    println!("The result from {author_query:?}");
    println!("{db_result:?}");

    Ok(())
}

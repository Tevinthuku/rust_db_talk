use anyhow::{anyhow, Context};
use connection_pool::DieselPool;
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    sql_query,
    sql_types::{Integer, Jsonb, Text},
};
use serde::Deserialize;

use crate::schema::{self, book};
use crate::schema::{author, book_author};

#[derive(
    Debug, Identifiable, Selectable, Queryable, QueryableByName, PartialEq, Eq, Hash, Clone,
)]
#[diesel(table_name = author)]
pub struct Author {
    id: i32,
    name: String,
}

#[tracing::instrument(skip(pool), level = "info")]
pub async fn author_details(
    pool: DieselPool,
    the_author_id: i32,
) -> anyhow::Result<Option<Author>> {
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Failed to get connection")?;
        author::table
            .find(the_author_id)
            .first::<Author>(&mut conn)
            .optional()
            .context("Failed to get author")
    })
    .await
    .map_err(|err| anyhow!(err))?
}

#[derive(Debug, Identifiable, Selectable, Queryable, QueryableByName, Deserialize)]
#[diesel(table_name = book)]
pub struct Book {
    id: i32,
    pub title: String,
}

#[derive(Identifiable, Selectable, Queryable)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = book_author)]
#[diesel(primary_key(book_id, author_id))]
pub struct BookAuthor {
    book_id: i32,
    author_id: i32,
}

#[derive(Debug, Deserialize, FromSqlRow, AsExpression)]
#[serde(transparent)]
#[diesel(sql_type = Jsonb)]
pub struct AuthorBooks(Vec<Book>);

/// Here's how you can implement FromSql for your types
/// https://docs.diesel.rs/master/src/diesel/pg/types/json.rs.html#29
impl FromSql<diesel::sql_types::Jsonb, Pg> for AuthorBooks {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }
        serde_json::from_slice(&bytes[1..]).map_err(|_| "Invalid Json".into())
    }
}

// with diesel's DSL (Domain specific language)
#[tracing::instrument(skip(pool), level = "info")]
pub async fn author_books_diesel_dsl(
    pool: DieselPool,
    the_author_id: i32,
) -> anyhow::Result<(Author, AuthorBooks)> {
    use crate::schema::author::dsl::*;
    use crate::schema::book::dsl::*;
    use crate::schema::book_author::dsl::*;
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Failed to get a connection")?;
        let aggregation_sql = "JSONB_AGG(book)";
        let result: (Author, AuthorBooks) = author
            .inner_join(book_author.inner_join(book))
            .select((
                author::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::Jsonb>(aggregation_sql),
            ))
            .group_by(schema::author::id)
            .filter(schema::author::id.eq(the_author_id))
            .first::<(Author, AuthorBooks)>(&mut conn)
            .context("Failed to fetch author with books")?;

        Ok(result)
    })
    .await?
}

#[derive(QueryableByName, Debug)]
pub struct AuthorWithBooks {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Jsonb)]
    pub books: AuthorBooks,
}

// with raw sql
#[tracing::instrument(skip(pool), level = "info")]
pub async fn author_books_raw_sql(
    pool: DieselPool,
    the_author_id: i32,
) -> anyhow::Result<AuthorWithBooks> {
    tokio::task::spawn_blocking(move || {
        let query = r#"
        SELECT 
            author.id, 
            author.name,
            JSONB_AGG("book") as books 
            FROM (author INNER JOIN (book_author INNER JOIN book ON (book_author.book_id = book.id)) 
            ON (book_author.author_id = author.id)) WHERE (author.id = $1)
         GROUP BY author.id
    "#;

        let mut conn = pool.get().context("Failed to get a connection")?;

        sql_query(query)
            .bind::<Integer, _>(the_author_id)
            .load::<AuthorWithBooks>(&mut conn)
            .context("Failed to get author with books")?
            .pop()
            .ok_or_else(|| anyhow!("Failed to get the author with books"))
    }).await.map_err(anyhow::Error::new)?
}

#[cfg(test)]
mod tests {
    use crate::fixtures::diesel_pool_fixture_with_single_author;
    use connection_pool::DieselPool;
    use rstest::*;

    use crate::author::{author_books_diesel_dsl, author_books_raw_sql};

    #[rstest]
    #[tokio::test]
    async fn test_impl_1_getting_author_books_works(
        diesel_pool_fixture_with_single_author: DieselPool,
    ) {
        let result = author_books_diesel_dsl(diesel_pool_fixture_with_single_author, 1).await;

        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_impl_2_getting_author_books_works(
        diesel_pool_fixture_with_single_author: DieselPool,
    ) {
        let result = author_books_raw_sql(diesel_pool_fixture_with_single_author, 1).await;
        assert!(result.is_ok())
    }
}

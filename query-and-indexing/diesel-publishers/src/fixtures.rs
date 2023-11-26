use diesel::{pg::Pg, ExpressionMethods, RunQueryDsl};
use rstest::*;

use connection_pool::{test_fixtures::diesel_pool_fixture, DieselPool};

use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use crate::schema::{author, book, book_author};
#[fixture]
pub fn diesel_pool_fixture_with_single_author(diesel_pool_fixture: DieselPool) -> DieselPool {
    let mut conn = diesel_pool_fixture.get().unwrap();
    run_migrations(&mut conn);
    let author_id = diesel::insert_into(author::table)
        .values(&author::name.eq("Test author"))
        .returning(author::id)
        .get_result::<i32>(&mut conn)
        .unwrap();
    let book_id = diesel::insert_into(book::table)
        .values(&book::title.eq("Test author book"))
        .returning(book::id)
        .get_result::<i32>(&mut conn)
        .unwrap();

    diesel::insert_into(book_author::table)
        .values((
            &book_author::author_id.eq(author_id),
            &book_author::book_id.eq(book_id),
        ))
        .execute(&mut conn)
        .unwrap();
    diesel_pool_fixture
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) {
    HarnessWithOutput::write_to_stdout(&mut *connection)
        .run_pending_migrations(MIGRATIONS)
        .unwrap();
}

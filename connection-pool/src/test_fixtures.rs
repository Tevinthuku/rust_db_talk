use diesel::{connection::SimpleConnection, r2d2::ConnectionManager};
use rstest::fixture;

use sqlx::{Connection, PgPool};

use crate::DieselPool;

#[fixture]
pub async fn sqlx_pool_fixture() -> PgPool {
    use sqlx::postgres::PgConnection;
    use sqlx::postgres::PgPoolOptions;
    dotenv_flow::dotenv_flow().ok();

    let db_url = std::env::var("TEST_DATABASE_URL").unwrap();

    let mut conn = PgConnection::connect(&db_url).await.unwrap();

    let db_name = uuid::Uuid::new_v4();

    sqlx::query(&format!(r#"CREATE DATABASE "{}""#, db_name))
        .execute(&mut conn)
        .await
        .unwrap();
    println!("Created database {db_name}");
    let db_url_with_new_db = format!("{db_url}/{db_name}");

    println!("New db_url {db_url_with_new_db}");

    PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url_with_new_db)
        .await
        .unwrap()
}

#[fixture]
pub fn diesel_pool_fixture() -> DieselPool {
    dotenv_flow::dotenv_flow().ok();
    use diesel::pg::PgConnection;
    use diesel::Connection;
    let db_url = std::env::var("TEST_DATABASE_URL").unwrap();

    let mut conn = PgConnection::establish(&db_url).unwrap();
    let db_name = uuid::Uuid::new_v4();

    conn.batch_execute(&format!(r#"CREATE DATABASE "{}""#, db_name))
        .unwrap();
    let db_url_with_new_db = format!("{db_url}/{db_name}");

    println!("New db_url {db_url_with_new_db}");

    let manager = ConnectionManager::<diesel::PgConnection>::new(db_url_with_new_db);
    diesel::r2d2::Builder::new()
        .max_size(2)
        .build(manager)
        .unwrap()
}

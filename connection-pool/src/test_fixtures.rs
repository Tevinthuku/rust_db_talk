use rstest::fixture;

use sqlx::PgPool;

#[fixture]
pub async fn sqlx_pool_fixture() -> PgPool {
    use sqlx::postgres::PgPoolOptions;
    dotenv_flow::dotenv_flow().ok();

    let db_url = std::env::var("TEST_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .unwrap();

    let db_name = uuid::Uuid::new_v4();

    sqlx::query(&format!(r#"CREATE DATABASE "{}""#, db_name))
        .execute(&pool)
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

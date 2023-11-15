// @generated automatically by Diesel CLI.

// Comment from @Tev -> diesel examined all tables and also included the _sqlx_migrations
// this can be filtered away in the `diesel.toml` file like so
// filter = { except_tables = ["_sqlx_migrations"] }

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    test_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    test_table_diesel (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(_sqlx_migrations, test_table, test_table_diesel,);

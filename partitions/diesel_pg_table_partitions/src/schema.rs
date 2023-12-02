// @generated automatically by Diesel CLI.

diesel::table! {
    sales (sale_id, product_id, sale_date) {
        sale_id -> Int4,
        product_id -> Int4,
        sale_date -> Date,
        amount -> Numeric,
    }
}

diesel::table! {
    sales_december_2023 (sale_id, product_id, sale_date) {
        sale_id -> Int4,
        product_id -> Int4,
        sale_date -> Date,
        amount -> Numeric,
    }
}

diesel::table! {
    sales_jan_2024 (sale_id, product_id, sale_date) {
        sale_id -> Int4,
        product_id -> Int4,
        sale_date -> Date,
        amount -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sales,
    sales_december_2023,
    sales_jan_2024,
);

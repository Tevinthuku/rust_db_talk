// @generated automatically by Diesel CLI.

diesel::table! {
    sales (sale_id, product_id, sale_date) {
        sale_id -> Int4,
        product_id -> Int4,
        sale_date -> Date,
        amount -> Numeric,
    }
}

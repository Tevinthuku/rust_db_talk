-- Add migration script here
-- Add up migration script here
CREATE TABLE product (
    product_sku TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    price NUMERIC(10, 2) NOT NULL
);


CREATE TABLE product_sale (
    product_sale_id SERIAL PRIMARY KEY,
    product_sku TEXT NOT NULL REFERENCES product(product_sku),
    quantity INT NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    sale_date TIMESTAMP NOT NULL
);


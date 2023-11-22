-- Add migration script here
CREATE TABLE product_inventory (
    product_sku TEXT PRIMARY KEY REFERENCES product(product_sku),
    quantity INT NOT NULL
)

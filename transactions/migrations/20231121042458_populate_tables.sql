-- Add migration script here
INSERT INTO product (product_sku, name, price) VALUES
    ('SKU001', 'Product A', 19.99),
    ('SKU002', 'Product B', 29.99),
    ('SKU003', 'Product C', 39.99);

-- Insert data into the product_sale table
INSERT INTO product_sale (product_sale_id, product_sku, quantity, price, sale_date) VALUES
    (1, 'SKU001', 5, 19.99, '2023-01-01 08:00:00'),
    (2, 'SKU002', 3, 29.99, '2023-01-02 10:30:00'),
    (3, 'SKU003', 8, 39.99, '2023-01-03 15:45:00');

-- Insert data into the product_inventory table
INSERT INTO product_inventory (product_sku, quantity) VALUES
    ('SKU001', 50),
    ('SKU002', 20),
    ('SKU003', 100);

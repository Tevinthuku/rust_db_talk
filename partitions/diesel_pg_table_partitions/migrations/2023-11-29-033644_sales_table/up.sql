-- Your SQL goes here
CREATE SEQUENCE sales_sale_id_seq;

CREATE TABLE sales (
    sale_id INT DEFAULT nextval('sales_sale_id_seq') NOT NULL,
    product_id INT NOT NULL,
    sale_date DATE NOT NULL,
    amount DECIMAL NOT NULL,
    -- unique constraint on partitioned table must include all partitioning columns
    -- https://stackoverflow.com/questions/71610180/postgresql-partition-table-unique-index-problem
    PRIMARY KEY(sale_id, product_id, sale_date)
) PARTITION BY RANGE (sale_date);

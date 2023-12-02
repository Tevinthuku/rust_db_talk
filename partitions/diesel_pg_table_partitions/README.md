## Setup

Run the following commands:

```
diesel setup
```

```
diesel migration run
```

## Creating a partition

```
cargo run -- create-partition sales_jan_2024 2024-01-01 2024-01-31
```

## inserting data (a new sale)

```
cargo run -- new-sale 1 2024-01-02 200
```

## Querying data (sales between a date range)

```
cargo run -- read-sales 2023-12-01 2023-12-30
```

## Analyze the query plan

```
EXPLAIN ANALYZE SELECT * FROM sales WHERE (sales.sale_date BETWEEN '2023-12-01' AND '2023-12-30')
```

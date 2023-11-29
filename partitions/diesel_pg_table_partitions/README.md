## Setup

```
diesel migration run
```

## Creating a partition

```
cargo run -- create-partition november_2023 2023-11-01 2023-11-30
```

## inserting data
```
cargo run -- new-sale 1 2023-11-02 200
```

## Querying data
```
cargo run -- read-sales 2023-11-01 2023-11-30
```


## Analyze the query plan

```
EXPLAIN ANALYZE SELECT * FROM sales WHERE (sales.sale_date BETWEEN '2023-11-01' AND '2023-11-30')
```
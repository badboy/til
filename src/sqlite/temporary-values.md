# Temporary values in SQLite

To select from some values:

```sql
WITH vals (k,v) AS (
    VALUES
    (0,-9999),
    (1, 100)
)
SELECT * FROM vals;
```


To actually create a temporary table:

```
CREATE TEMP TABLE temp_table AS                                     
WITH t (k, v) AS (
 VALUES
 (0, -99999),
 (1, 100)
)
SELECT * FROM t;
```

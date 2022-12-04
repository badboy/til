# Recursive Queries

[SQLite documentation](https://sqlite.org/lang_with.html).

Generate a series of integers, per ID, with a given `min` and `max`.

```sql
CREATE TABLE data AS
WITH t (id, min, max) AS (
 VALUES
  (1, 4, 6),
  (2, 6, 6),
  (3, 7, 9)
)
SELECT * FROM t;

WITH RECURSIVE exp AS (
  SELECT
    id, min, max, min as x
  FROM data
  UNION ALL
  SELECT
    id, min, max, x+1 as x
  FROM
    exp
  WHERE x < max
)

SELECT * from exp ORDER BY id;
```

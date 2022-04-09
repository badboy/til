# Working with dates

Full docs: [Date And Time Functions](https://sqlite.org/lang_datefunc.html)

## Datetime of now

```sql
SELECT datetime('now');
```

## Timestamp to datetime

```sql
SELECT datetime(1092941466, 'unixepoch');
```

## Datetime to timestamp

```sql
SELECT strftime('%s', 'now');
```

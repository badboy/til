# Working with dates

Full docs: [Date And Time Functions](https://sqlite.org/lang_datefunc.html)

## Datetime of now

```
SELECT datetime('now');
```

## Timestamp to datetime

```
SELECT datetime(1092941466, 'unixepoch');
```

## Datetime to timestamp

```
SELECT strftime('%s', 'now');
```

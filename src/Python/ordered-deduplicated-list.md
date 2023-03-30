# Deduplicate a list and keep the order

```python
duplicated_list = [1,1,2,1,3,4,1,2,3,4]
ordered = list(dict.fromkeys(duplicated_list)) # => [1, 2, 3, 4]
```

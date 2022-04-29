# The date of Easter

```python
def easter(year):
    y = year;
    c = y//100;
    n = y-19*(y//19);
    k = (c-17)//25;
    i = c-c//4-(c-k)//3+19*n+15;
    i = i-30*(i//30);
    i = i-(i//28)*(1-(i//28)*(29//(i+1))*((21-n)//11));
    j = y+y//4+i+2-c+c//4;
    j = j-7*(j//7);
    l = i-j;
    m = 3+(l+40)//44;
    d = l+28-31*(m//4);

    return (m, d)


print(easter(2021))
```

Based on the wonderful explanation in [§3. Calendrical.](https://ganelson.github.io/inweb/foundation-module/3-tm.html#SP3) of the Inform7 documentation

# Changes after updating home-manager

Using [`nvd`](https://gitlab.com/khumba/nvd), diff the latest 2 home-manager generations:

```
home-manager generations | head -n 2 | cut -d' ' -f 7 | tac | xargs nvd diff
```

Of course only gets you the changes after they are installed.

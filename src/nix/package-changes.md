# Changes after updating home-manager

Using [`nvd`](https://gitlab.com/khumba/nvd), diff the latest 2 home-manager generations:

```
home-manager generations | head -n 2 | cut -d' ' -f 7 | tac | xargs nvd diff
```

Of course only gets you the changes after they are installed.

This is also built-in now using `nix store diff-closures`:

```
$ nix store diff-closures /nix/store/xfy75lrmsh23hj2c8kzqr4n1cfvzh1s2-home-manager-generation /nix/store/5rdyvvk6jngd2hd44bsa14bpzxraigbi-home-manager-generation
gnumake: 4.4.1 → ∅, -1546.9 KiB
home-manager: -9.0 KiB
```

via Nix Manual: [nix store diff-closures](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-store-diff-closures.html)

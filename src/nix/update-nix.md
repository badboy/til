# Update nix

```
nix upgrade-nix
```

This might require to be run as `root`:

```
sudo -i nix upgrade-nix
```

`-i` to inherit the environment and have `nix` actually available in the `$PATH`

Sources:

* https://github.com/DeterminateSystems/nix-installer/issues/421
* https://github.com/DeterminateSystems/nix-installer/issues/508
* https://github.com/DeterminateSystems/nix-installer/issues/596

# home-manager: Allow unfree packages

Some packages are `unfree`, due to their licenses, e.g. `android-studio`.
To use them one needs to allow unfree packages.

In a home-manager flake this can be done as follows.

In one of your modules add:

```nix
{ pkgs, ... }: {
  nixpkgs = {
    config = {
      allowUnfree = true;
      allowUnfreePredicate = (_: true);
    };
  };
}
```

The `allowUnfreePredicate` is due to [home-manager#2942](https://github.com/nix-community/home-manager/issues/2942) (I haven't actually checked that it is necessary)

# A minimal flake for a shell

```nix
{
  description = "A very basic flake";

  outputs = { self, nixpkgs, ... }:
    let
      supportedSystems = [ "aarch64-linux" "aarch64-darwin" "x86_64-darwin" "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs;
              [
                ripgrep
              ];
          };
        });
    };
}
```

Start a shell with:

```
nix develop
```

---

Old way:

A `shell.nix` with

```nix
with import <nixpkgs> {};

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    ripgrep
  ];
}
```

And run it with

```
nix-shell
```

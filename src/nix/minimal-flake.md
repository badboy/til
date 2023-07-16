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
                lima
                nixpkgs-fmt
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

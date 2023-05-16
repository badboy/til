# List all available attributes of a flake

List all names of packages for a certain target architecture:

```
$ nix eval .#packages.x86_64-linux --apply builtins.attrNames --json
["default"]
```

List all supported architectures of a package:

```
$ nix eval .#packages --apply builtins.attrNames --json
["aarch64-darwin","aarch64-linux","i686-linux","x86_64-darwin","x86_64-linux"]
```

List everything from a flake:

```
$ nix eval . --apply builtins.attrNames --json
["__darwinAllowLocalNetworking","__ignoreNulls","__impureHostDeps","__propagatedImpureHostDeps","__propagatedSandboxProfile","__sandboxProfile","__structuredAttrs","all","args","buildInputs","buildPhase","builder","cargoArtifacts","cargoVendorDir","checkPhase","cmakeFlags","configureFlags","configurePhase","depsBuildBuild","depsBuildBuildPropagated","depsBuildTarget","depsBuildTargetPropagated","depsHostHost","depsHostHostPropagated","depsTargetTarget","depsTargetTargetPropagated","doCheck","doInstallCargoArtifacts","doInstallCheck","drvAttrs","drvPath","inputDerivation","installPhase","mesonFlags","meta","name","nativeBuildInputs","out","outPath","outputName","outputs","overrideAttrs","passthru","patches","pname","propagatedBuildInputs","propagatedNativeBuildInputs","src","stdenv","strictDeps","system","type","userHook","version"]
```

(via [garnix: steps](https://garnix.io/docs/steps))

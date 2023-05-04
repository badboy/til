# Replacing/Adding another cache server

Add the following to `/etc/nix/nix.conf`:

```
substituters = https://aseipp-nix-cache.freetls.fastly.net?priority=10 https://cache.nixos.org
```

`cache.nixos.org` is the default, with priority 40.
See [the reference manual](https://nixos.org/manual/nix/stable/command-ref/conf-file.html#conf-substituters).
Lower value means higher priority.
According to that the substituters are only used when called by a trusted user or in a trusted substituter list.

According to <https://nixos.wiki/wiki/Maintainers:Fastly#Beta_.2B_IPv6_.2B_HTTP.2F2> the `aesipp-nix-cache` is new.
It's unclear from when that info is and what the current status of this project is (as of May 2023).

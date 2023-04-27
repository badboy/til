# Remote Builds

After reading [Using Nix with Dockerfiles][nix-with-dockerfiles] I wanted to understand how to use `dockerTools.buildImage` to build a Docker image,
instead of relying on a `Dockerfile`.

The issue: I'm on a M1 MacBook, an aarch64 machine.
The Docker on this machine runs within an aarch64 Linux VM.
Naively building a nix flake means it will build for aarch64 macOS and that then cannot run within the Docker container.
So I needed to understand how to either cross-compile or use a remote builder.

I went for the latter, using my `x86_64-linux` server with nix installed as a remote builder.

[nix-with-dockerfiles]: https://mitchellh.com/writing/nix-with-dockerfiles

I started with the test command from the [Remote Builds](https://nixos.org/manual/nix/stable/advanced-topics/distributed-builds.html) docs, slightly modified:

```
nix build --impure --expr '(with import <nixpkgs> { system = "x86_64-linux"; }; runCommand "foo" {} "uname > $out")' --builders 'ssh://builder x86_64-linux' --max-jobs 0 -vvv
```

`--max-jobs 0` will ensure it won't run any local tasks
and `-vvv` will show all the debug output.

This starts downloading nixpkgs and stuff and then ... fail:

```
error: unable to start any build; either increase '--max-jobs' or enable remote builds.
       https://nixos.org/manual/nix/stable/advanced-topics/distributed-builds.html
```

Unhelpful.
The `-vvv` was necessary to even get any understanding of what's failing.
Close to the top one can see this:

```
ignoring the client-specified setting 'builders', because it is a restricted setting and you are not a trusted user
```

[The docs about `trusted-users`](https://nixos.org/manual/nix/stable/command-ref/conf-file.html#conf-trusted-users) say that adding users there essentially give that user root rights.
So let's not do that and instead configure the builder machine in `/etc/nix/machines`:

```
ssh://builder x86_64-linux
```

I also needed to set the user and the SSH key:

```
ssh://jer@builder?ssh-key=/Users/jer/.ssh/id_ed25519
```

Apparently `builders = @/etc/nix/machines` is the default,
but if not you can set that in `/etc/nix/nix.conf`.
After that a restart of the nix daemon will be necessary:

```
sudo launchctl kickstart -k system/org.nixos.nix-daemon
```

Re-running the `nix build --impure ...` will fail again:

```
error: unexpected end-of-file
error: builder for '/nix/store/6ji85w7v51fs3x21szvbgmx4dj0vpjqs-foo.drv' failed with exit code 1;
       last 10 log lines:
       [...]
       > error: you are not privileged to build input-addressed derivations
       [...]
       > debug1: Exit status 1
       For full logs, run 'nix log /nix/store/6ji85w7v51fs3x21szvbgmx4dj0vpjqs-foo.drv'.
```

Sounds very similar to the initial issue.
This time I set `trusted-users = jer` in `/etc/nix/nix.conf` on the builder machine.
Then restarted the nix daemon with:

```
systemctl restart nix-daemon
```

Now the `nix build` on macOS succeeds and:

```
$ cat result
Linux
```

## Building Docker images for `x86_64`

Last but not least I can build the Docker image for `x86_64` now.
The full example is in [`github:badboy/flask-nix-example`](https://github.com/badboy/flask-nix-example).

```
nix build '.#packages.x86_64-linux.dockerImage' --max-jobs 0
```

Then load it:

```
docker load < result
```

And finally run the container:

```
docker run -it --rm -p 5001:5000 --platform linux/amd64 flask-example
```

---

## Resources

* [Using Nix with Dockerfiles](https://mitchellh.com/writing/nix-with-dockerfiles)
* [NixOS/hydra#584: you are not privileged to build derivations](https://github.com/NixOS/hydra/issues/584)
* Nix Reference Manual: [Nix configuration file: trusted-users](https://nixos.org/manual/nix/stable/command-ref/conf-file.html#conf-trusted-users)
* Nix Reference Manual: [Distributed Builds](https://nixos.org/manual/nix/stable/advanced-topics/distributed-builds.html)
* [`github:badboy/flask-nix-example`](https://github.com/badboy/flask-nix-example)

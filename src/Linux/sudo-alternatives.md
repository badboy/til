# sudo alternatives

How to gain privileges to run commands as another user (most likely `root`).

* [doas](https://man.openbsd.org/doas) ([repository](https://github.com/Duncaen/OpenDoas))
  * from the BSD world
* [really](https://manpages.debian.org/stretch/chiark-really/really.8.en.html)
  * minimal suid binary, checking only a user's group and write access to a file
* [sudo-rs](https://github.com/memorysafety/sudo-rs)
  * sudo reimplementation in Rust
* [run0](https://www.freedesktop.org/software/systemd/man/devel/run0.html)
  * systemd-powered tool, based on systemd-run and polkit
* [userv](https://www.chiark.greenend.org.uk/~ian/userv/)
  * old unix tool

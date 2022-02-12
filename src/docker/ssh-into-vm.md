# SSH into the Docker VM on macOS

Run `socat` first:

```
socat -d -d ~/Library/Containers/com.docker.docker/Data/debug-shell.sock pty,rawer
```

This will print some lines, including the PTY device opened, like

```
PTY is /dev/ttys029
```

Use that to connect using `screen`:

```
screen /dev/ttys029
```

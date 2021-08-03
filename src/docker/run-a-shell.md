# Run a shell with a Docker image

```
docker run -t -i --rm ubuntu:20.04 bash
```

Changing the platform, e.g. to use `x86_64` when running on an M1 MacBook:

```
docker run -t -i --rm --platform linux/amd64 ubuntu:20.04 bash
```

# Docker on a remote host

```
docker context create remote --docker "host=ssh://hostname"
docker context use remote
```

## Is the docker daemon running?

If the Docker daemon is _not_ running on the remote host, you might see this error message:

```
Cannot connect to the Docker daemon at http://docker.example.com. Is the docker daemon running?
```

The `docker.example.com` host is of course nonsense.
The solution: Start the Docker daemon on the remote host and it should work.

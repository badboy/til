# Match and act on query parameters

In order to serve different files based on query parameters, first create a map:

```
map $query_string $resource_name {
    ~resource=alice$ alice;
    ~resource=bob$ bob;
}
```

Then in your `server` block match the location and inside rewrite the URL the way you need it:

```
location = /.well-known/thing {
    root  /var/www/thing;

    if ($resource_name) {
      rewrite ^(.*)$ /$resource_name.json break;
    }

    try_files $uri = 404;
}
```

Now if someone requests `/.well-known/thing?resource=alice` nginx will serve `/var/www/thing/alice.json`.

# Set and return a cookie

nginx can directly set and read a cookie.
This snippet sets a cookie `i_was_here` to the value `hello_world` and also renders a cookie it received into a JSON payload:

```
location = /cookies {
    add_header Content-Type application/json;
    add_header Set-Cookie i_was_here=hello_world;
    return 200 '{"cookie": "$cookie_i_was_here" }';
}
```

Test it with:

```
curl https://example.com/cookies -H 'Cookie: i_was_here=hello_world'
```

Note that the cookie is client-supplied, so it might contain whatever and there's no escape happening,
so something like the following request will result in invalid JSON:

```
curl https://example.com/cookies -H 'Cookie: i_was_here=foo" test'
```

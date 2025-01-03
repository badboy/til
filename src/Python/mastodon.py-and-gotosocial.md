# mastodon.py and GoToSocial

Using [Mastodon.py] against a [GoToSocial] server.
The setup was a bit finicky, so I better document it.

[GoToSocial]: https://gotosocial.org/
[mastodon.py]: https://mastodonpy.readthedocs.io/en/stable/

It's somewhat explained [in the documentation](https://mastodonpy.readthedocs.io/en/stable/index.html),
but GoToSocial made it extra special.

## Register an app

Run:

```python
from mastodon import Mastodon

Mastodon.create_app(
    'til-app',
    api_base_url = 'https://social.berline.rs',
    to_file = 'gotosocial_client.secret'
)
```

This creates a file `berliners.secret`.
It returns the `client_id` and `client_secret`.
That's also stored in the file.

This step only needs to be done once!

## Get an acces token via OAuth

```python
mastodon = Mastodon(client_id = 'gotosocial_client.secret',)
url = mastodon.auth_request_url()
```

Open the returned URL in a browser and log in.

Copy the returned out-of-band token, then log in using that:

```python
mastodon.log_in(
    'janerik+whatrustisit@fnordig.de',
    code=oob_token,
    to_file = 'gotosocial_user.secret'
)
```

This returns the access token and also writes it to the file `gotosocial_user.secret`
Store this access token.

## Create an authenticated API client

```python
mastodon = Mastodon(
    access_token=MASTO_ACCESS_TOKEN,
    api_base_url=MASTO_URL,
    client_id=MASTO_CLIENT_ID,
    client_secret=MASTO_CLIENT_SECRET,
    version_check_mode="none"
)
```

## Check that it's working

```python
mastodon.account_verify_credentials()
```

## Post a status

```python
mastodon.status_post("It works!")
```

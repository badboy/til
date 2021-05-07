# GitHub Webhooks

GitHub can send webhooks to a configured server on events.
By default this is done on any push event to the repository.

GitHub attaches an HMAC signature using the provided secret, 
which allows to verify that the content is really coming from GitHub.
Documentation about this is available in [Securing your webhooks](https://docs.github.com/en/developers/webhooks-and-events/securing-your-webhooks).

In Rust one can verify the signature like this:

```rust
use hex::FromHex;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;


fn authenticate(key: &str, content: &[u8], signature: &str) -> bool {
    // https://developer.github.com/webhooks/securing/#validating-payloads-from-github
    const SIG_PREFIX: &str = "sha256=";
    let sans_prefix = signature[SIG_PREFIX.len()..].as_bytes();
    match Vec::from_hex(sans_prefix) {
        Ok(sigbytes) => {
            let mut mac =
                HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
            mac.update(content);
            mac.verify(&sigbytes).is_ok()
        }
        _ => false,
    }
}
```

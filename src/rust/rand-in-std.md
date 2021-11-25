# Random values using only libstd

```rust
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
let random_value = RandomState::new().build_hasher().finish() as usize;
```

[via `ffi-support`](https://github.com/mozilla/ffi-support/blob/0fdc22a8dfe3731be5fd39b311e4e4885219e26c/src/handle_map.rs#L1065-L1069)

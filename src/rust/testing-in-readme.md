# Testing code blocks in the README

via [github.com/artichoke/intaglio](https://github.com/artichoke/intaglio/blob/6eb8e325da6d537423c58704dd9e4d399d5ef821/src/lib.rs#L69)

```rust
// Ensure code blocks in README.md compile
#[cfg(doctest)]
macro_rules! readme {
    ($x:expr) => {
        #[doc = $x]
        mod readme {}
    };
    () => {
        readme!(include_str!("../README.md"));
    };
}
#[cfg(doctest)]
readme!();
```

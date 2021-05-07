# [Today I Learned](https://fnordig.de/til)

A collection of snippets, thoughts and notes about stuff I learned.

## Generate book

This project uses [mdbook](https://github.com/rust-lang/mdBook) to render a website.
To generate the book:

```
cargo xtask
mdbook build
```

The project is automatically deployed on pushes to this repository.
The table of contents, index files and the overview are updated automatically.

## References

* [simonw/til](https://github.com/simonw/til)
* [jbranchaud/til](https://github.com/jbranchaud/til)

## License

The deployment code is distributed under the terms of the MIT license. See [LICENSE](LICENSE).

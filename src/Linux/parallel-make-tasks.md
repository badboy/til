# Runing parallel tasks from make

With the combination of multiple tools, you can serve static files over HTTP and rerun a build step whenever any input file changes.

I use these tools:

* [https](https://crates.io/crates/https) - static file server
* [fd](https://crates.io/crates/fd-find) - a faster `find`
* [entr](https://github.com/eradman/entr) - run arbitrary commands when files change
* make

With this `Makefile`:

```makefile
default:
	$(MAKE) MAKEFLAGS=--jobs=2 dev
.PHONY: default

dev: serve rerun
.PHONY: dev

build:
	# Put your build task here.
	# I generate a book using https://github.com/rust-lang/mdBook
	mdbook build
.PHONY: build

serve: build
	@echo "Served on http://localhost:8000"
	# Change to the generate build directory, then serve it.
	cd _book && http
.PHONY: serve

rerun:
	# fd respects your `.gitignore`
	fd | entr -s 'make build'
.PHONY: rerun
```

All it takes to continously serve and build the project is:

```
make
```

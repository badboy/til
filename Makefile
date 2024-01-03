default:
	rm -rf _book
	$(MAKE) MAKEFLAGS=--jobs=2 dev
.PHONY: default

dev: serve rerun
.PHONY: dev

build:
	mdbook build
.PHONY: build

serve: build
	@echo "Served on http://localhost:8000"
	@open "http://localhost:8000"
	cd _book && httplz
.PHONY: serve

rerun:
	fd | entr -s 'make build'
.PHONY: rerun

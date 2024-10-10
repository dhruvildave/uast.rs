.DEFAULT_GOAL := all
.PHONY: all format clean test docker install cover

all:
	cargo b -v -r

format:
	rustfmt -v **/*.rs

clean:
	cargo clean

docker:
	docker build --pull -f Containerfile -t uast . && docker system prune -f

install:
	cargo install -v --path .

test:
	cargo t -v

cover:
	[ -f "./bin/cargo-tarpaulin" ] || cargo install cargo-tarpaulin --root .
	./bin/cargo-tarpaulin --exclude-files src/main.rs

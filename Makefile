.DEFAULT_GOAL := all
.PHONY: all format clean test docker podman install cover msrv

all:
	cargo b -v -r

format:
	rustfmt -v **/*.rs

clean:
	cargo clean
	rm -rf bin/

docker:
	docker build --pull -f Containerfile -t uast . && docker system prune -f

podman:
	podman build --pull -f Containerfile -t uast . && podman system prune -f

install:
	cargo install -v --path .

test:
	cargo t -v

cover:
	[ -f "./bin/cargo-tarpaulin" ] || cargo install cargo-tarpaulin --root .
	./bin/cargo-tarpaulin --exclude-files src/main.rs

msrv:
	[ -f "/bin/cargo-msrv" ] || cargo install cargo-msrv --root .
	./bin/cargo-msrv find

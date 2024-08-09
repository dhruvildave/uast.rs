.DEFAULT_GOAL := all
.PHONY: all format clean test

all:
	cargo b -v -r

format:
	rustfmt -v **/*.rs

clean:
	cargo clean

test:
	cargo test -v

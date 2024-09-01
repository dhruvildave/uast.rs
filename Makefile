.DEFAULT_GOAL := all
.PHONY: all format clean test docker install

all:
	cargo b -v -r

format:
	rustfmt -v **/*.rs

clean:
	cargo clean

docker:
	docker build --pull -t uast . && docker system prune -f

install:
	cargo install -v --path .

test:
	cargo t -v

.DEFAULT_GOAL := all
.PHONY: all format clean test docker

all:
	cargo b -v -r

format:
	rustfmt -v **/*.rs

clean:
	cargo clean

docker:
	docker build -t uast . && docker system prune -f

test:
	cargo t -v

.PHONY: help ready docker-build docker-ready fmt lint test

help:
	@cat Makefile | grep -E "^\w+$:"

ready: fmt lint test

run: ready
	@echo "===> Running Rust Application"
	cargo run

docker-build:
	docker build --no-cache . -t app_$(notdir $(shell pwd))

docker-ready: docker-build
	docker run -t app_$(notdir $(shell pwd))


clean:
	@echo "===> Cleaning"
	cargo clean

fmt:
	@echo "===> Formatting"
	cargo fmt

lint:
	@echo "===> Linting"
	cargo clippy

test:
	@echo "===> Testing EVERYTHING"
	cargo test


.PHONY: help ready docker-build docker-ready fmt lint test

help:
	@cat Makefile | grep -E "^\w+$:"

ready: fmt lint test

docker-build:
	docker build . -t nneji123/workatastartup-scraper

docker-ready: docker-build
	docker run -v ./:/app -t nneji123/workatastartup-scraper


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

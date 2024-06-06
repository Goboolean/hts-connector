.PHONY: test build lint
test:
	cargo test --verbose

build:
	cargo build --verbose

lint:
	cargo clippy --all-targets --all-features -- -D warnings
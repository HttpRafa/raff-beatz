.PHONY: fix

## Fix target
fix:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features

## Build target
build: 
	trunk build --release
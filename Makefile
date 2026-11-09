.PHONY: build ci

build: test.so
	cargo build

test.so: test.c cutil
	gcc test.c -shared -fPIC -rdynamic -I./cutil -o test.so

run: build
	cargo run

ci: build
	cargo fmt
	cargo clippy --all-targets
	cargo test

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all

.PHONY: run
run:
	cargo +nightly-2020-10-05 build --release 
	target/release/zeropool-substrate-node --dev --tmp

.PHONY: build
build:
	cargo +nightly-2020-10-05 build  --release 


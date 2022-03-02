run-debug:
	cargo run -- --dev

run:
	cargo run --release -- --dev

toolchain:
	./scripts/init.sh

build:
	cargo build --release

check:
	SKIP_WASM_BUILD= cargo check --all --tests

test:
	SKIP_WASM_BUILD= cargo test --all

t:
	cargo test -p pallet-rmrk-core -- --nocapture && \
	cargo test -p pallet-rmrk-market -- --nocapture && \
	cargo test -p pallet-rmrk-equip -- --nocapture

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

init: toolchain build-full

benchmark-output-core:
	cargo run --manifest-path node/Cargo.toml --release --features runtime-benchmarks -- benchmark --extrinsic '*' --pallet pallet_rmrk_core --output runtime/src/weights/pallet_rmrk_core.rs --execution=wasm --wasm-execution=compiled

test-benchmark-core:
	cargo test --manifest-path pallets/rmrk-core/Cargo.toml --features runtime-benchmarks -- --nocapture
all: init build


init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain install nightly --component clippy --component rustfmt
	@rustup target add wasm32-unknown-unknown --toolchain nightly

build:
	@echo ⚙️ Building a release...
	@cargo b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

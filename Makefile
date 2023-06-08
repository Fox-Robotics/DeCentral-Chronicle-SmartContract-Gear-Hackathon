.PHONY: all build fmt fmt-doc init lint pre-commit test full-test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

fmt-doc:
	@echo ⚙️ Format the docs...
	@cargo fmt -- --config wrap_comments=true,format_code_in_doc_comments=true

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain install nightly --component clippy --component rustfmt
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo t

full-test:
	@echo ⚙️ Running all tests...
	@cargo t -- --include-ignored
build:
    @find public -name "*.br" -type f -delete
    dx build --release

tag: check build
	#!/usr/bin/env nu
	git tag (open Cargo.toml | get package.version)

check:
    @find public -name "*.br" -type f -delete
    # dx fmt --check --all-code
    dx check
    cargo clippy --target wasm32-unknown-unknown -- -D warnings

build:
    dx build --release

tag: check build
	#!/usr/bin/env nu
	git tag (open Cargo.toml | get package.version)

check:
    dx fmt --check --all-code
    dx check
    cargo clippy -- -D warnings

tag:
	#!/usr/bin/env nu
	git tag (open Cargo.toml | get package.version)

# Copyright (C) 2023 Nitrokey GmbH
# SPDX-License-Identifier: CC0-1.0

src/se05x/commands.rs: src/se05x/commands.toml generate_commands.py
	python generate_commands.py src/se05x/commands.toml src/se05x/commands.rs
	rustfmt --edition 2021 src/se05x/commands.rs

verify-commands:
	python generate_commands.py src/se05x/commands.toml target/commands_diff.rs
	rustfmt --edition 2021 target/commands_diff.rs
	diff target/commands_diff.rs src/se05x/commands.rs

check: src/se05x/commands.rs
	cargo c

lint: src/se05x/commands.rs verify-commands
	cargo c
	cargo fmt --check
	cargo clippy

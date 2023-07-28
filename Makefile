# Copyright (C) 2023 Nitrokey GmbH
# SPDX-License-Identifier: CC0-1.0

src/se050/commands.rs: src/se050/commands.toml generate_commands.py
	python generate_commands.py src/se050/commands.toml src/se050/commands.rs
	rustfmt --edition 2021 src/se050/commands.rs

verify-commands:
	python generate_commands.py src/se050/commands.toml target/commands_diff.rs
	rustfmt --edition 2021 target/commands_diff.rs
	diff target/commands_diff.rs src/se050/commands.rs

check: src/se050/commands.rs
	cargo c

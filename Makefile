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
	cargo c --features builder
	cargo c --features nrf,nrf-hal-common/52840 --target thumbv7em-none-eabihf
	cargo c --features lpc55 --target thumbv8m.main-none-eabi


lint: src/se05x/commands.rs verify-commands
	cargo c
	cargo fmt --check
	cargo clippy
	cargo clippy --features nrf,nrf-hal-common/52840 --target thumbv7em-none-eabihf
	cargo clippy --features lpc55 --target thumbv8m.main-none-eabi

README.md: src/lib.rs Makefile
	grep '//!' src/lib.rs |grep -v '//! # ' | sed 's/^...//g' | sed 's/^ //g' > README.md

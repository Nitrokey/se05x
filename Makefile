# Copyright (C) 2023 Nitrokey GmbH
# SPDX-License-Identifier: CC0-1.0

GENERATED_FILES := src/se05x/commands.rs README.md

src/se05x/commands.rs: src/se05x/commands.toml generate_commands.py
	python generate_commands.py src/se05x/commands.toml src/se05x/commands.rs
	rustfmt --edition 2021 src/se05x/commands.rs

.PHONY: verify-commands
verify-commands:
	mkdir -p target
	python generate_commands.py src/se05x/commands.toml target/commands_diff.rs
	rustfmt --edition 2021 target/commands_diff.rs
	diff target/commands_diff.rs src/se05x/commands.rs

.PHONY: check
check: src/se05x/commands.rs
	cargo c
	cargo c --features builder
	cargo c --features defmt
	cargo c --features nrf,nrf-hal-common/52840 --target thumbv7em-none-eabihf
	cargo c --features lpc55-v0.3 --target thumbv8m.main-none-eabi
	cargo c --features lpc55-v0.4 --target thumbv8m.main-none-eabi


.PHONY: lint
lint: src/se05x/commands.rs verify-commands
	reuse lint
	cargo c
	cargo fmt --check
	cargo clippy
	cargo clippy --features nrf,nrf-hal-common/52840 --target thumbv7em-none-eabihf
	cargo clippy --features lpc55-v0.3 --target thumbv8m.main-none-eabi
	cargo clippy --features lpc55-v0.4 --target thumbv8m.main-none-eabi
	cargo doc --features aes-session,builder,serde --no-deps

.PHONY: test
test:
	cargo t
	cargo t --features builder,serde_bytes,defmt
	cargo t --no-default-features

.PHONY: semver-checks
semver-checks:
	 cargo semver-checks --only-explicit-features --features aes-session,builder,lpc55

README.md: src/lib.rs Makefile
	# REUSE-IgnoreStart
	echo '<!-- Copyright (C) 2023 Nitrokey GmbH -->' > README.md
	echo '<!-- SPDX-License-Identifier: LGPL-3.0-only -->' >> README.md
	# REUSE-IgnoreEnd
	grep '//!' src/lib.rs |grep -v '//! # ' | sed 's/^...//g' | sed 's/^ //g' >> README.md

.PHONY: ci
ci: export RUSTFLAGS=-Dwarnings
ci: export RUSTDOCFLAGS=-Dwarnings
ci: check lint
	$(MAKE) --always-make $(GENERATED_FILES)
	git diff --exit-code -- $(GENERATED_FILES)

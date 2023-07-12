src/se050/commands.rs: src/se050/commands.toml generate_commands.py
	python generate_commands.py src/se050/commands.toml src/se050/commands.rs
	cat src/se050/commands.rs

verify-commands:
	python generate_commands.py src/se050/commands.toml target/commands_diff.rs
	diff target/commands_diff.rs src/se050/commands.rs


check: src/se050/commands.rs
	cargo c

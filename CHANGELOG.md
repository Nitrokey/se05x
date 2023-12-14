<!--
Copyright (C) 2023 Nitrokey GmbH
SPDX-License-Identifier: CC0-1.0
-->

# Changelog

## [v0.1.2][] (2023-12-14 )

### Changed

- Use `&dyn` to reduce the reliance on monomorphization and therefore save in binary size.
- Update typed-builder to version `0.18.0`
- Improve documentation on [docs.rs](https://docs.rs/se05x)

### Fixed

- Clippy warning

[v0.1.2]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.2

## [v0.1.1][] (2023-11-24 )

Initial [crates.io](https://crates.io) release.

This release contains a T=1 driver and the definition for most commands for the SE050 secure element.

[v0.1.1]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.1

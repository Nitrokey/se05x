<!--
Copyright (C) 2023 Nitrokey GmbH
SPDX-License-Identifier: CC0-1.0
-->

# Changelog


## [v0.1.8][] (2025-03-06)

- Add support for `defmt` behind a `defmt` feature

## [v0.1.7][] (2024-10-25)

- Add support for `lpc55-hal` 0.4 behind a `lpc55-v0.4` feature

[v0.1.7]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.7

## [v0.1.6][] (2025-08-13)

- Fix `ExportObject` for large keys (NIST-P521 and brainpoolp512) ([#16][])

[#16]: https://github.com/Nitrokey/se03x/pull/16
[v0.1.6]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.6


## [v0.1.5][] (2025-06-06)

- Implement Debug, PartialEq, Eq, Clone and Copy on all relevant structs ([#15][])

[#15]: https://github.com/Nitrokey/se03x/pull/15
[v0.1.5]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.5

## [v0.1.4][] (2024-05-27)

### Added

- `ReadEcCurveListResponse`: `is_set` to get wether a curve is set or not ([#13][])

### Changed

- Added deprecation warning to `create_and_set_curve`  ([#13][])

[#13]: https://github.com/Nitrokey/se03x/pull/13
[v0.1.4]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.4

## [v0.1.3][] (2023-04-11)

### Added

- Added `create_and_set_curve_params` to aid the compiler optimize-out curve constants ([#5][])
- Fix rustdoc warnings ([#6][])
- Fix README example ([#7][])

### Fixed

- Removed unneeded `impl` block in codegen ([#4][])
- Fix lints ([#3][])
- Removed unused byteorder dependency ([#3][])
- Use latest serde-bytes rather than serde-byte-array ([#3][])

[#7]: https://github.com/Nitrokey/se07x/pull/7
[#6]: https://github.com/Nitrokey/se06x/pull/6
[#5]: https://github.com/Nitrokey/se05x/pull/5
[#4]: https://github.com/Nitrokey/se04x/pull/4
[#3]: https://github.com/Nitrokey/se03x/pull/3
[v0.1.3]: https://github.com/Nitrokey/se05x/releases/tag/v0.1.3

## [v0.1.2][] (2023-12-14)

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

# Audit Summary (Pre-publication)

## Changes made

- Added crate metadata required for crates.io (description, license, repository, readme, keywords, categories).
- Added dual-license files (MIT and Apache-2.0) and an initial changelog.
- Added `#![forbid(unsafe_code)]` and `#![warn(missing_docs)]` to the crate.
- Clarified units and bearing conventions in API documentation and README.
- Added longitude wrapping and input clamping helpers for numerical robustness.
- Added tests for antimeridian wrapping and polar edge cases.

## Known limitations

- Uses a spherical Earth model only (no WGS84 ellipsoid support).
- Repository URL in `Cargo.toml` is a placeholder and must be updated before publishing.
- `no_std` is not enabled yet because the crate currently relies on `std` trigonometry.

## Commands to run before publishing

```bash
cargo fmt
cargo test
cargo check
cargo clippy --all-targets --all-features
cargo package --allow-dirty
```

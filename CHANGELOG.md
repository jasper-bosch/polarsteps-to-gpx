# Changelog


## Unreleased


## 0.1.3 - 2026-05-17

### Fixed

- Support floating-point Unix timestamps in locations.json

### Changed

- Switched to [anyhow](https://crates.io/crates/anyhow) to handle errors


## 0.1.2 - 2025-06-01

### Changed

- Make `input` and `output` arguments optional. Look for `locations.json` in current directory when `input` is empty. See [#2](https://github.com/jasper-bosch/polarsteps-to-gpx/issues/2)

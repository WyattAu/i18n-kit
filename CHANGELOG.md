# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

## [0.1.3] - 2026-09-12

### Added

- config-knob behavior matrix: tests/config_matrix.rs pins the Translator knobs — default_locale changes the fallback target, the exact→language→default→key fallback chain order, catalog insert/overwrite changes output, bulk insert matches individual inserts, and params change interpolation. Also fixes two clippy violations in tests/json.rs (unused import, unwrap without allow).


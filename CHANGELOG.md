# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/0809android/japanese-text/compare/v0.2.0...v0.2.1) - 2026-05-09

### Other

- Document release workflow

## [0.2.0](https://github.com/0809android/japanese-text/compare/v0.1.1...v0.2.0) - 2026-05-09

### Added

- Add full-width katakana to half-width katakana conversion
- Add Unicode normalization helpers for NFC, NFD, NFKC, and NFKD
- Add dakuten and handakuten composition/decomposition helpers
- Add punctuation, bracket, quote, symbol, old kanji, and variation selector normalization
- Add `normalize`, `normalize_with_options`, `NormalizeOptions`, and `Normalizer`
- Add ASCII token preservation for URLs, email addresses, and number-like tokens
- Add character type ratios, script-mixing detection, Japanese extraction, ASCII extraction, and symbol removal
- Add property-based tests for round-trip conversion behavior

### Changed

- Use `unicode-normalization` for standards-based Unicode normalization
- Expand half-width katakana dakuten support for `ﾜﾞ` and `ｦﾞ`
- Preserve whitespace when removing symbols and punctuation
- Update README and examples for the expanded API surface

## [0.1.1](https://github.com/0809android/japanese-text/compare/v0.1.0...v0.1.1) - 2026-05-09

### Other

- Clarify test coverage documentation
- Verify crates before release publishing
- Automate crate releases with release-plz
- Fix iteration mark expansion
- Add CI and trusted publishing workflow

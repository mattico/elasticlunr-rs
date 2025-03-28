# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.3] - 2025-03-16
### Changed
 - Rewrote the English stemmer for improved performance ([#48](https://github.com/mattico/elasticlunr-rs/pull/48))
 - Changed the rust-version in Cargo.toml to match the Minimum Supported Rust Version (1.60.0).
 - Added support for either Criterion 0.4.0 or 0.5.0
 - Updated the Cargo.toml license specification to better match the situation described in the README.

## [3.0.2] - 2023-03-17
### Changed
 - Updated Minimum Supported Rust Version to 1.60.0.
 - Updated Criterion to 0.4.0.

### Added
 - Language support for Korean ([#50](https://github.com/mattico/elasticlunr-rs/pull/50))
 - Language support for Hungarian ([#51](https://github.com/mattico/elasticlunr-rs/pull/51))

## [3.0.1] - 2022-07-23
### Changed
 - Updated dependencies and MSRVs to fix builds. ([#47](https://github.com/mattico/elasticlunr-rs/pull/47) et. al.)

## [3.0.0] - 2022-06-01
### Added
 - Language support for Arabic ([#40](https://github.com/mattico/elasticlunr-rs/pull/40])).
 - Add the `Language` trait to make it easier to implement languages outside the crate.
 - Add `IndexBuilder::add_field_with_tokenizer` to specify the tokenizer for a field.

### Changed
 - Update to 2018 edition, and bump MSRV to 1.54.0.
 - Change benchmarks to use Criterion.
 - Remove dependency on lazy_static.
 - Update dependencies.
 - Use Unicode character classes for trimmer.
 - `IndexBuilder` functions which add fields will now panic if the same field is added multiple times.
 - Fix `IndexBuilder` not respecting field insertion order.

### Removed
 - Remove the `default` feature. You now need to opt-in to the `languages` feature.
 - Remove the deprecated function `Pipeline::for_language`.
 - Remove the `pipeline::tokenize*` functions, which are now implemented as part of the `Language` trait.
 - Remove `Index::add_doc_with_tokenizer(s)`, replaced by `IndexBuilder::add_field_with_tokenizer`.
 - Remove the `Language` enum. Use the `Language` trait implementations in the `lang` modules, and the free functions `lang::from_name`, `lang::from_code`, and `lang::languages`.


[Unreleased]: https://github.com/mattico/elasticlunr-rs/compare/v3.0.3...HEAD
[3.0.0]: https://github.com/mattico/elasticlunr-rs/compare/v2.3.14...v3.0.0
[3.0.1]: https://github.com/mattico/elasticlunr-rs/compare/v3.0.0...v3.0.1
[3.0.2]: https://github.com/mattico/elasticlunr-rs/compare/v3.0.1...v3.0.2
[3.0.3]: https://github.com/mattico/elasticlunr-rs/compare/v3.0.2...v3.0.3

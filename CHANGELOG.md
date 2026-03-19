# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-03-19

### Added
- `--timeout` flag to kill programs that hang (default: 30 seconds)
- Library API (`difftest` crate) exposing `run_pair`, `InputSource`, `RunResult`, `make_diff`, and `shell_words` for programmatic use
- Doc comments on all public API items

### Changed
- Restructured as lib + bin crate (was bin-only)
- `run_pair` now takes a `Duration` timeout parameter

## [0.1.0] - 2026-03-14

### Added
- Initial release: run two programs with the same inputs, compare outputs
- Auto-generated smoke inputs when no `--inputs` flag provided
- Stdin piping mode (`--stdin`)
- Stderr comparison (`--stderr`)
- Quiet mode (`-q`)
- Colored diff output
- CI-friendly exit codes

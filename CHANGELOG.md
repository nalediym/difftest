# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-03-19

### Added
- You can now set `--timeout 10` to kill programs that hang (default: 30 seconds)
- Use difftest as a Rust library — `run_pair()`, `InputSource`, `RunResult`, `make_diff`, and `shell_words` are all public
- `bin/test-lane` runs the full quality gauntlet: check, fmt, clippy, test, doc, wasm32 compile, audit

### Changed
- Restructured as lib + bin crate — you can now `use difftest::run_pair` in your own code
- `run_pair` takes a `Duration` timeout parameter

### For contributors
- Doc comments on all public API items
- Library compiles to wasm32-unknown-unknown

## [0.1.0] - 2026-03-14

### Added
- Initial release: run two programs with the same inputs, compare outputs
- Auto-generated smoke inputs when no `--inputs` flag provided
- Stdin piping mode (`--stdin`)
- Stderr comparison (`--stderr`)
- Quiet mode (`-q`)
- Colored diff output
- CI-friendly exit codes

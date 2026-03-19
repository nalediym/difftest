# difftest

[![Crates.io](https://img.shields.io/crates/v/difftest.svg)](https://crates.io/crates/difftest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-7%20passing-brightgreen.svg)]()

**Differential testing for the AI rewrite era.**

You rewrote something — a function, a service, an entire codebase. Maybe you did it by hand. Maybe AI did it for you. Either way: **does it still work the same?**

difftest runs two programs with the same inputs and compares their outputs. If they match, the rewrite is behaviorally equivalent. If they don't, you see exactly where they diverge.

<p align="center">
  <img src="demo/hero.gif" alt="difftest demo" width="800">
</p>

## Install

```bash
cargo install difftest
```

Or build from source:

```bash
git clone https://github.com/nalediym/difftest
cd difftest
cargo build --release
```

## Usage

```bash
# Auto-generate smoke test inputs
difftest ./old ./new

# Custom inputs (each value is one test case)
difftest ./old ./new --inputs "hello" "world" ""

# Compare programs written in different languages
difftest "python3 old.py" "bun new.ts" --inputs test

# Pipe stdin to both
cat data.txt | difftest ./old ./new --stdin

# Also compare stderr
difftest ./old ./new --stderr

# Quiet mode — just pass/fail, no diffs
difftest ./old ./new -q

# Set a timeout (default: 30s) — kills programs that hang
difftest ./old ./new --timeout 10
```

## What it compares

For each input, difftest runs both programs and checks three things:

| Check | Default | Flag |
|-------|---------|------|
| **stdout** | Always compared | — |
| **exit code** | Always compared | — |
| **stderr** | Ignored | `--stderr` |
| **timeout** | 30 seconds | `--timeout N` |

If all checks match: **PASS**. If any differ: **FAIL** with a line-by-line diff.

## Use cases

### AI rewrites
You asked Claude to rewrite your Python service in Rust. difftest tells you if the output is identical.

### Language migrations
Moving from Python to Elixir? Express to Hono? Rails to Phoenix? Run both versions side-by-side.

### Refactoring
You rewrote a gnarly function to be cleaner. difftest catches behavioral regressions your tests might miss.

### Compiler testing
Like [Csmith](https://github.com/csmith-project/csmith) — compile the same code with two compilers, compare outputs.

### Binary verification
Extract capabilities from a binary with [Ursula](https://github.com/nalediym/ursula), then verify the extraction is correct:
```bash
ursula extract /usr/bin/wc
difftest /usr/bin/wc ./wc.shell/target/debug/wc
```

## Library usage

difftest is also a Rust library. Add it to your `Cargo.toml`:

```toml
[dependencies]
difftest = "0.2"
```

Then use it programmatically:

```rust
use difftest::{run_pair, InputSource, RunResult};
use std::time::Duration;

let input = InputSource::Args(vec!["hello".into()]);
let timeout = Duration::from_secs(30);

match run_pair("./old", "./new", &input, false, timeout) {
    RunResult::Pass { label } => println!("{label}: identical"),
    RunResult::Fail { label, stdout_diff, .. } => {
        println!("{label}: diverged");
        if let Some(diff) = stdout_diff {
            print!("{diff}");
        }
    }
    RunResult::Error { label, message } => eprintln!("{label}: {message}"),
}
```

## How it works

```
          ┌─────────────┐
          │   Inputs     │
          │ (args/stdin) │
          └──────┬───────┘
                 │
        ┌────────┴────────┐
        ▼                 ▼
  ┌──────────┐     ┌──────────┐
  │ Program A │     │ Program B │
  │ (oracle)  │     │(candidate)│
  └─────┬─────┘     └─────┬─────┘
        │                 │
        ▼                 ▼
  ┌──────────┐     ┌──────────┐
  │  stdout   │     │  stdout   │
  │  stderr   │ ══► │  stderr   │  compare
  │  exit code│     │  exit code│
  └──────────┘     └──────────┘
        │                 │
        └────────┬────────┘
                 ▼
          PASS or FAIL
```

No AI. No magic. Just: run both, compare everything, show the diff.

## CI integration

difftest exits with code 0 on success, 1 on failure — works in any CI pipeline:

```yaml
# GitHub Actions
- name: Verify rewrite
  run: difftest ./original ./rewrite --inputs "test1" "test2"
```

## Prior art

Differential testing is a well-established technique. difftest packages it into a CLI that just works.

| Tool | Scope | Complexity |
|------|-------|-----------|
| **difftest** | Any two CLI programs | `cargo install`, zero config |
| [Csmith](https://github.com/csmith-project/csmith) | C compiler testing | Generates random C programs |
| [DIFFER](https://github.com/trailofbits/differ) | Debloated program validation | Python, YAML configs, libfuzzy-dev |
| [Diferencia](https://github.com/lordofthejars/diferencia) | HTTP microservice comparison | HTTP-only |
| `diff <(./a) <(./b)` | One-off comparison | No batching, no exit codes, no reporting |

## Design principles

- **Zero config** — no YAML, no TOML, no setup. Two arguments and go.
- **Language agnostic** — compares programs, not code. Python vs Rust? Fine.
- **CI friendly** — exit codes, quiet mode, structured output.
- **Minimal dependencies** — `clap` for arg parsing, `libc` for process timeout. ~500 lines of Rust.

## Development

```bash
# Run the full quality gauntlet (check, fmt, clippy, test, doc, wasm32)
bin/test-lane

# Quick mode (check + test only)
bin/test-lane --quick

# Verify the library compiles to WebAssembly
bin/test-lane --wasm
```

## License

[MIT](LICENSE)

---

Built by [Naledi](https://github.com/nalediym). See [CHANGELOG](CHANGELOG.md) for release history.

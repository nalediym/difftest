# Use Case: lipgloss (Go) vs lipgloss-rs (Rust)

A real-world differential test comparing Charm's [lipgloss](https://github.com/charmbracelet/lipgloss) (Go) against [lipgloss-rs](https://github.com/whit3rabbit/lipgloss-rs) (Rust port).

## What This Tests

Both programs do the same thing:
- Accept a name as a CLI argument (default "World")
- Print a styled greeting header with a rounded border
- Print a small info table (name, language, framework)
- Use the same styling: bold, ANSI-256 colors, rounded borders

The Go program uses `github.com/charmbracelet/lipgloss` v1.1.0.
The Rust program uses `lipgloss` from `whit3rabbit/lipgloss-rs`.

## Build

```bash
# Go
cd go-example && go build -o go-example .

# Rust
cd rust-example && cargo build --release
```

## Run difftest

```bash
difftest ./go-example/go-example ./rust-example/target/release/rust-example \
  --inputs "Naledi" "World" "" "Test"
```

## Results

**0/4 passed, 4 failed** — the programs are NOT behaviorally equivalent.

### Differences Found

difftest caught three categories of behavioral divergence:

#### 1. TTY Detection (biggest finding)

The Go lipgloss library detects when stdout is not a TTY (i.e., piped) and **strips all ANSI escape codes**, outputting plain text. The Rust port **always emits ANSI escape codes** regardless of whether output is piped.

This means:
- Go output (piped): `Hello, Naledi!` (plain text, no color codes)
- Rust output (piped): `\x1b[1;38;2;255;95;175;48;2;38;38;38mHello, Naledi!\x1b[0m` (full ANSI)

This is a significant behavioral difference for any downstream tooling that consumes the output (log files, test harnesses, CI pipelines).

#### 2. Expected Content Differences

The "Language" and "Framework" rows intentionally differ (`Go`/`lipgloss` vs `Rust`/`lipgloss-rs`). These are expected and confirm difftest correctly reports content divergence.

#### 3. Text Alignment Padding

Even ignoring ANSI codes, there's a subtle difference in how "Hello, World!" is centered within the 40-character bordered box. The Go version pads as `Hello, World!··` (trailing spaces) while the Rust version pads as `·Hello, World!·` (more evenly distributed). This suggests a difference in the center-alignment algorithm between the two implementations.

### What This Reveals About difftest

- difftest successfully catches byte-level output differences between real-world programs
- It clearly shows which test inputs pass vs fail
- The diff output makes it easy to spot ANSI escape code differences
- It would benefit from an optional `--strip-ansi` flag for comparing styled terminal output where you only care about the text content, not the escape sequences

### What This Reveals About lipgloss-rs

Despite claiming "1:1 API compatibility" and "exact rendering parity," the Rust port differs from the Go original in at least two ways:
1. No TTY-aware color profile downgrading (always emits truecolor ANSI)
2. Slightly different center-alignment padding distribution

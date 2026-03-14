# difftest

Run two programs with the same inputs, compare outputs.

You rewrote something. **Does it still work the same?**

```
$ difftest ./old-server ./new-server

difftest  comparing two programs

  A (oracle):     ./old-server
  B (candidate):  ./new-server

  Running 7 test cases...

  PASS  (no args)
  PASS  hello
  PASS  Hello, World!
  PASS
  PASS  42
  PASS  -1
  PASS  a b c

  ✓ 7/7 passed — programs are behaviorally equivalent
```

## Install

```bash
cargo install difftest
```

## Usage

```bash
# Auto-generate smoke test inputs
difftest ./old ./new

# Custom inputs (each --inputs value is one test case)
difftest ./old ./new --inputs "hello" "world" ""

# Compare programs written in different languages
difftest "python3 old.py" "python3 new.py" --inputs test

# Pipe stdin to both
cat data.txt | difftest ./old ./new --stdin

# Also compare stderr
difftest ./old ./new --stderr

# Quiet mode (no diffs, just pass/fail)
difftest ./old ./new -q
```

## What it compares

For each input, difftest runs both programs and checks:

- **stdout** — identical output?
- **exit code** — same return code?
- **stderr** *(opt-in with `--stderr`)* — identical error output?

If all match → `PASS`. If any differ → `FAIL` with a diff.

## When to use this

- You asked AI to rewrite your code
- You're migrating from Python to Rust (or any language to any language)
- You refactored a function and want to verify behavior
- You're testing compiler output (like [Csmith](https://github.com/csmith-project/csmith))
- You extracted a [Shell](https://github.com/nalediym/ursula) from a binary and want proof it works

## Prior art

The concept of differential testing has existed since the 1990s. difftest packages it into a CLI that just works.

- [Csmith](https://github.com/csmith-project/csmith) — differential testing for C compilers
- [DIFFER](https://github.com/trailofbits/differ) — validates debloated programs (requires Python, YAML configs, libfuzzy-dev)
- [Diferencia](https://github.com/lordofthejars/diferencia) — HTTP microservice comparison

## License

MIT

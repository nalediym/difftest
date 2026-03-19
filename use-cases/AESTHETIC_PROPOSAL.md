# difftest Aesthetic Proposal

Research date: 2026-03-14

## Current State

difftest uses raw ANSI escape codes throughout `main.rs`. No styling library.
Only dependency beyond `clap` is nothing -- all formatting is manual `\x1b[...m` sequences.

### Current format strings (verbatim from source)

**Header:**
```
\x1b[1m\x1b[35mdifftest\x1b[0m  comparing two programs
  \x1b[2mA (oracle):\x1b[0m    \x1b[36m{program_a}\x1b[0m
  \x1b[2mB (candidate):\x1b[0m  \x1b[36m{program_b}\x1b[0m
  \x1b[2mRunning {} test case{}...\x1b[0m
```
Renders as: bold magenta "difftest", dim labels, cyan program paths.

**Pass/Fail lines:**
```
  \x1b[1m\x1b[32mPASS\x1b[0m  {label}
  \x1b[1m\x1b[31mFAIL\x1b[0m  {label}
  \x1b[1m\x1b[33mERR \x1b[0m  {label} — {message}
```

**Diff output (inside FAIL):**
```
        \x1b[2mexit code:\x1b[0m A={}, B={}
        \x1b[2mstdout:\x1b[0m
          \x1b[31m{minus_line}\x1b[0m     (lines starting with -)
          \x1b[32m{plus_line}\x1b[0m      (lines starting with +)
```

**Summary:**
```
  \x1b[1m\x1b[32m✓\x1b[0m {passed}/{total} passed — \x1b[32mprograms are behaviorally equivalent\x1b[0m
  \x1b[1m\x1b[31m✗\x1b[0m {passed}/{total} passed, \x1b[31m{failed} failed\x1b[0m, \x1b[33m{errored} errors\x1b[0m
```

### What it looks like today (approximate render)

```
difftest  comparing two programs

  A (oracle):    ./old
  B (candidate):  ./new

  Running 7 test cases...

  PASS  (no args)
  PASS  hello
  FAIL  Hello, World!
        exit code: A=0, B=1
        stdout:
          -Hello, World!
          +Hello World!
  PASS  (empty)
  PASS  42
  PASS  -1
  PASS  a b c

  ✗ 6/7 passed, 1 failed
```

### Problems with current output

1. **No visual grouping** -- pass/fail lines run together with no separation
2. **Diff is hard to scan** -- just indented text, no box or border around it
3. **No progress feedback** -- all results appear at once after all tests run
4. **Labels blend in** -- test input labels use default color, same weight as everything else
5. **Summary is bare** -- single line, no timing info, no visual weight
6. **Raw ANSI codes are unmaintainable** -- `\x1b[1m\x1b[35m` scattered everywhere

---

## Proposed Improvements

### 1. Color Scheme

| Element              | Current               | Proposed                    |
|----------------------|-----------------------|-----------------------------|
| Tool name "difftest" | Bold magenta          | Bold magenta (keep)         |
| Program paths        | Cyan                  | Cyan bold                   |
| Labels (oracle, etc) | Dim white             | Dim white (keep)            |
| PASS                 | Bold green            | Green bold (keep)           |
| FAIL                 | Bold red              | Red bold (keep)             |
| ERR                  | Bold yellow           | Yellow bold (keep)          |
| Test input labels    | Default (no color)    | **Dim** for pass, **white bold** for fail |
| Diff minus lines     | Red                   | Red with `- ` gutter prefix in dim |
| Diff plus lines      | Green                 | Green with `+ ` gutter prefix in dim |
| Diff context lines   | Default               | Dim                         |
| Summary pass         | Green ✓               | Green bold, inside a rounded border |
| Summary fail         | Red ✗                 | Red bold, inside a rounded border |
| Timing               | Not shown             | Dim, right of summary       |

**Rationale:** The core red/green/yellow palette is correct and conventional. The improvements
are about using **dim** more aggressively for secondary information and **bold** for scannable
anchors. This matches ursula's verify command, which dims labels and bolds status indicators.

**Crate:** `console` (same as ursula uses) -- provides `style("text").green().bold()` etc.
Alternative: `owo-colors` (lighter weight, `"text".green().bold()` via traits).

### 2. Typography

| Technique  | Where to use                                      |
|------------|--------------------------------------------------|
| **Bold**   | Tool name, PASS/FAIL/ERR badges, summary verdict |
| **Dim**    | Labels, passing test names, context diff lines, timing, separator lines |
| **Italic** | Not used (poor terminal support)                  |
| Underline  | Not used                                          |

**New: PASS/FAIL badges with fixed-width alignment.**
Currently PASS is 4 chars and ERR has a trailing space to match. Make all badges exactly
4 chars wide with consistent padding:

```
   PASS  (no args)
   FAIL  Hello, World!
   ERR!  ./missing-binary — not found
```

**Crate:** Same styling crate handles this. No extra dependency.

### 3. Layout

#### a. Bordered diff blocks

Wrap diff output in a light box using Unicode box-drawing characters, inspired
by lipgloss's `RoundedBorder` (corners: `╭╮╰╯`, edges: `─│`). This visually
separates diff content from the test list.

```
   FAIL  Hello, World!
   ╭─ stdout ──────────────────────────╮
   │  - Hello, World!                  │
   │  + Hello World!                   │
   ╰───────────────────────────────────╯
```

When both stdout and exit code differ:

```
   FAIL  Hello, World!
   ╭─ exit code ───────────────────────╮
   │  A (oracle):     0                │
   │  B (candidate):  1                │
   ╰───────────────────────────────────╯
   ╭─ stdout ──────────────────────────╮
   │  - Hello, World!                  │
   │  + Hello World!                   │
   ╰───────────────────────────────────╯
```

The border foreground should be **dim** so it frames without competing.

**Crate:** Implement manually with box-drawing chars (simple), or use `tabled` crate
for structured table rendering. Manual is preferred to keep dependencies minimal.

#### b. Section spacing

Add a blank line between the test results list and the summary.
Add a thin dim separator line before the summary:

```
   PASS  (no args)
   PASS  hello
   FAIL  Hello, World!
   ...

   ── summary ──────────────────────────

   ✗ 6/7 passed, 1 failed        0.03s
```

#### c. Consistent indentation

Current indentation uses 2-space prefix (`"  "`). Keep this, but ensure diff
blocks are indented by 5 spaces (aligned under the test label, past the badge).

### 4. Progress Indicators

#### a. Inline progress (low-effort, high-value)

Print each test result **as it completes** rather than collecting all results
and printing at the end. The current code already does this -- the `for r in &results`
loop prints immediately. But the test *execution* is collected first via
`inputs.iter().map(...).collect()`. Change this to run-and-print in a single loop:

```rust
for input in &inputs {
    let r = runner::run_pair(...);
    // print immediately
    match &r { ... }
    results.push(r);
}
```

This gives the user feedback as tests run, which matters for slow programs.

#### b. Counter prefix (medium effort)

Show a running count: `[3/7]` before each result line.

```
   [1/7] PASS  (no args)
   [2/7] PASS  hello
   [3/7] FAIL  Hello, World!
```

**Crate:** None needed. Pure formatting.

#### c. Spinner (optional, higher effort)

Show a spinner while each test case is running. This requires a background
thread or async. Only worth it if test programs can be slow (network, I/O).

**Crate:** `indicatif` -- provides spinners, progress bars, multi-progress.
Ursula uses the `console` crate (same author as `indicatif`). These pair well together.

**Recommendation:** Start with (a) run-and-print. Add (b) counter prefix. Skip (c) spinner
unless users report slow test programs as a pain point.

### 5. Summary Section

#### Current
```
  ✓ 7/7 passed — programs are behaviorally equivalent

  ✗ 6/7 passed, 1 failed
```

#### Proposed

**All pass:**
```
   ── summary ──────────────────────────

   ✓ 7/7 passed                  0.03s
   programs are behaviorally equivalent
```

**With failures:**
```
   ── summary ──────────────────────────

   ✗ 6/7 passed, 1 failed        0.03s

   Failed tests:
     Hello, World!
```

**With errors:**
```
   ── summary ──────────────────────────

   ✗ 5/7 passed, 1 failed        0.03s
   ⚠ 1 error

   Failed tests:
     Hello, World!
   Errors:
     (stdin) — B failed to execute: not found
```

Changes:
- **Elapsed time** in dim, right-aligned (use `std::time::Instant` -- no crate needed)
- **Failed test list** repeated at the bottom so users don't have to scroll up
- **Separator line** above the summary for visual weight
- **Equivalence message** on its own line, dim, only on full pass
- **Error count** on separate line in yellow when errors exist

---

## Crate Recommendations

| Crate        | Purpose                        | Size    | Used by ursula? |
|--------------|--------------------------------|---------|-----------------|
| `console`    | Styled text, terminal width    | ~100KB  | Yes             |
| `indicatif`  | Spinners, progress bars        | ~200KB  | No              |
| `owo-colors` | Lighter alternative to console | ~30KB   | No              |

**Recommendation:** Use `console` for consistency with ursula. It handles:
- `style("PASS").green().bold()`
- `style("label").dim()`
- Terminal width detection (for right-aligning timing)
- Color support detection (auto-disables when piped)

Avoid `colored` crate -- it's unmaintained. Avoid `termcolor` -- verbose API.

If minimal dependencies are preferred, `owo-colors` is much smaller and provides
the same styling via Rust traits (`"PASS".green().bold()`). It does not include
terminal width detection, so you'd need `terminal_size` crate separately.

---

## Mockup: Full improved output

```
difftest  comparing two programs

  A (oracle):     ./old
  B (candidate):  ./new

  Running 7 test cases...

  [1/7] PASS  (no args)
  [2/7] PASS  hello
  [3/7] FAIL  Hello, World!
        ╭─ exit code ───────────────────╮
        │  A (oracle):     0            │
        │  B (candidate):  1            │
        ╰───────────────────────────────╯
        ╭─ stdout ──────────────────────╮
        │  - Hello, World!              │
        │  + Hello World!               │
        ╰───────────────────────────────╯
  [4/7] PASS  (empty)
  [5/7] PASS  42
  [6/7] PASS  -1
  [7/7] PASS  a b c

  ── summary ───────────────────────────

  ✗ 6/7 passed, 1 failed         0.03s

  Failed tests:
    Hello, World!
```

**Color key for the mockup above:**
- `difftest` -- bold magenta
- `./old`, `./new` -- cyan bold
- `A (oracle):`, `B (candidate):` -- dim
- `Running 7 test cases...` -- dim
- `[1/7]` -- dim
- `PASS` -- green bold
- `FAIL` -- red bold
- Test labels after PASS -- dim
- Test labels after FAIL -- white (default weight)
- Box borders (`╭─╮│╰╯`) -- dim
- Diff `-` lines -- red
- Diff `+` lines -- green
- Section labels inside boxes (`stdout`, `exit code`) -- dim bold
- `── summary ──...` -- dim
- `✗` -- red bold
- `6/7 passed` -- default
- `1 failed` -- red bold
- `0.03s` -- dim, right-aligned
- `Failed tests:` / listed names -- dim

---

## Implementation Priority

1. **Replace raw ANSI with `console` crate** -- eliminates `\x1b[...]` codes, gets automatic
   color detection for free. Low effort, high maintainability win.
2. **Run-and-print loop** -- move from collect-then-print to print-as-you-go. Trivial change.
3. **Add elapsed time** -- `Instant::now()` at start, `elapsed()` at end. No new dep.
4. **Counter prefix `[n/N]`** -- simple format change.
5. **Dim passing test labels** -- one-line change with `console`.
6. **Bordered diff blocks** -- moderate effort, big readability improvement.
7. **Summary separator + failed test recap** -- moderate effort, useful for large test sets.
8. **Spinner** -- only if justified by user feedback. Requires `indicatif`.

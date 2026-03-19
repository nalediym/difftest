//! Differential testing library — run two programs with the same inputs, compare outputs.
//!
//! This crate provides the core engine behind the `difftest` CLI. You can use it
//! programmatically to compare the behavior of two programs across a set of inputs.

use std::io::Read;
use std::process::Command;
use std::time::Duration;

/// How to feed input to the programs under test.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum InputSource {
    /// Pass these as CLI arguments.
    Args(Vec<String>),
    /// Pipe stdin from the parent process.
    Stdin,
}

/// Result of comparing one test case across both programs.
#[derive(Debug)]
#[non_exhaustive]
pub enum RunResult {
    /// Both programs produced identical output and exit code.
    Pass {
        /// Human-readable label for this test case.
        label: String,
    },
    /// The programs diverged on output or exit code.
    Fail {
        /// Human-readable label for this test case.
        label: String,
        /// Unified-style diff of stdout, if it differed.
        stdout_diff: Option<String>,
        /// Unified-style diff of stderr, if it differed.
        stderr_diff: Option<String>,
        /// Exit code of program A.
        exit_a: i32,
        /// Exit code of program B.
        exit_b: i32,
    },
    /// One or both programs could not be executed.
    Error {
        /// Human-readable label for this test case.
        label: String,
        /// Description of what went wrong.
        message: String,
    },
}

struct ProgramOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

/// Parse a program string into command + args.
/// Supports "python old.py" -> ("python", ["old.py"])
fn parse_program(prog: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = prog.split_whitespace().collect();
    if parts.is_empty() {
        (prog, vec![])
    } else {
        (parts[0], parts[1..].to_vec())
    }
}

fn run_program(
    prog: &str,
    input: &InputSource,
    stdin_data: Option<&[u8]>,
    timeout: Duration,
) -> Result<ProgramOutput, String> {
    let (cmd, base_args) = parse_program(prog);

    let mut command = Command::new(cmd);
    command.args(&base_args);

    // Add input args
    if let InputSource::Args(args) = input {
        command.args(args);
    }

    // Handle stdin
    if stdin_data.is_some() {
        command.stdin(std::process::Stdio::piped());
    }

    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    let mut child = command.spawn().map_err(|e| e.to_string())?;

    // Write stdin if needed — then drop the handle so the child sees EOF.
    if let Some(data) = stdin_data {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(data);
        }
    }

    // Poll with try_wait until the child exits or the timeout expires.
    let poll_interval = Duration::from_millis(50);
    let start = std::time::Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(_status)) => {
                // Child exited — collect output.
                let output = child.wait_with_output().map_err(|e| e.to_string())?;
                return Ok(ProgramOutput {
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    exit_code: output.status.code().unwrap_or(-1),
                });
            }
            Ok(None) => {
                // Still running — check timeout.
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait(); // reap the zombie
                    return Err(format!("timed out after {}s", timeout.as_secs()));
                }
                std::thread::sleep(poll_interval);
            }
            Err(e) => return Err(e.to_string()),
        }
    }
}

/// Run both programs with the same input and compare their outputs.
///
/// Returns a [`RunResult`] indicating whether the programs matched, diverged,
/// or encountered an error. When `compare_stderr` is true, stderr is included
/// in the comparison. The `timeout` duration limits how long each individual
/// program is allowed to run before being killed.
pub fn run_pair(
    prog_a: &str,
    prog_b: &str,
    input: &InputSource,
    compare_stderr: bool,
    timeout: Duration,
) -> RunResult {
    let label = match input {
        InputSource::Args(args) if args.is_empty() => "(no args)".to_string(),
        InputSource::Args(args) => args.join(" "),
        InputSource::Stdin => "(stdin)".to_string(),
    };

    // Get stdin data if needed
    let stdin_data = match input {
        InputSource::Stdin => {
            let mut buf = Vec::new();
            if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
                return RunResult::Error {
                    label,
                    message: format!("Failed to read stdin: {e}"),
                };
            }
            Some(buf)
        }
        _ => None,
    };

    // Run A
    let a = match run_program(prog_a, input, stdin_data.as_deref(), timeout) {
        Ok(output) => output,
        Err(e) => {
            return RunResult::Error {
                label,
                message: format!("A failed to execute: {e}"),
            };
        }
    };

    // Run B
    let b = match run_program(prog_b, input, stdin_data.as_deref(), timeout) {
        Ok(output) => output,
        Err(e) => {
            return RunResult::Error {
                label,
                message: format!("B failed to execute: {e}"),
            };
        }
    };

    // Compare
    let stdout_match = a.stdout == b.stdout;
    let stderr_match = !compare_stderr || a.stderr == b.stderr;
    let exit_match = a.exit_code == b.exit_code;

    if stdout_match && stderr_match && exit_match {
        RunResult::Pass { label }
    } else {
        let stdout_diff = if !stdout_match {
            Some(make_diff(&a.stdout, &b.stdout))
        } else {
            None
        };

        let stderr_diff = if !stderr_match {
            Some(make_diff(&a.stderr, &b.stderr))
        } else {
            None
        };

        RunResult::Fail {
            label,
            stdout_diff,
            stderr_diff,
            exit_a: a.exit_code,
            exit_b: b.exit_code,
        }
    }
}

/// Produce a simple line-by-line diff between two strings.
///
/// Lines unique to `a` are prefixed with `-`, lines unique to `b` with `+`,
/// and matching lines with a space.
pub fn make_diff(a: &str, b: &str) -> String {
    let mut result = String::new();

    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();

    let max = a_lines.len().max(b_lines.len());

    for i in 0..max {
        let a_line = a_lines.get(i);
        let b_line = b_lines.get(i);

        match (a_line, b_line) {
            (Some(a), Some(b)) if a == b => {
                result.push_str(&format!(" {a}\n"));
            }
            (Some(a), Some(b)) => {
                result.push_str(&format!("-{a}\n"));
                result.push_str(&format!("+{b}\n"));
            }
            (Some(a), None) => {
                result.push_str(&format!("-{a}\n"));
            }
            (None, Some(b)) => {
                result.push_str(&format!("+{b}\n"));
            }
            (None, None) => {}
        }
    }

    result
}

/// Simple word splitting on spaces (respects quotes).
///
/// Handles double and single quotes but does not support escape characters.
pub fn shell_words(s: &str) -> Vec<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return vec![];
    }

    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in trimmed.chars() {
        if in_quote {
            if c == quote_char {
                in_quote = false;
            } else {
                current.push(c);
            }
        } else if c == '"' || c == '\'' {
            in_quote = true;
            quote_char = c;
        } else if c == ' ' {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }

    words
}

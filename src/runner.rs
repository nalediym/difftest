use std::io::Read;
use std::process::Command;

/// How to feed input to the programs.
pub enum InputSource {
    /// Pass these as CLI arguments.
    Args(Vec<String>),
    /// Pipe stdin from the parent process.
    Stdin,
}

/// Result of comparing one test case.
pub enum RunResult {
    Pass {
        label: String,
    },
    Fail {
        label: String,
        stdout_diff: Option<String>,
        stderr_diff: Option<String>,
        exit_a: i32,
        exit_b: i32,
    },
    Error {
        label: String,
        message: String,
    },
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

/// Run both programs with the same input, compare outputs.
pub fn run_pair(
    prog_a: &str,
    prog_b: &str,
    input: &InputSource,
    compare_stderr: bool,
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
    let a = match run_program(prog_a, input, stdin_data.as_deref()) {
        Ok(output) => output,
        Err(e) => {
            return RunResult::Error {
                label,
                message: format!("A failed to execute: {e}"),
            };
        }
    };

    // Run B
    let b = match run_program(prog_b, input, stdin_data.as_deref()) {
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

struct ProgramOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn run_program(
    prog: &str,
    input: &InputSource,
    stdin_data: Option<&[u8]>,
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

    // Write stdin if needed
    if let Some(data) = stdin_data {
        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(data);
        }
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    Ok(ProgramOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    })
}

/// Simple line-by-line diff showing A vs B.
fn make_diff(a: &str, b: &str) -> String {
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

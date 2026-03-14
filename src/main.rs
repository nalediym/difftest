mod runner;

use clap::Parser;
use runner::{InputSource, RunResult};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "difftest",
    about = "Run two programs with the same inputs, compare outputs",
    version,
    after_help = "Examples:\n  difftest ./old ./new\n  difftest ./old ./new --inputs hello world \"\"\n  difftest \"python old.py\" \"python new.py\" --inputs test\n  cat data.txt | difftest ./old ./new --stdin"
)]
struct Cli {
    /// First program (the oracle / original)
    program_a: String,

    /// Second program (the candidate / rewrite)
    program_b: String,

    /// Input arguments to pass to both programs
    #[arg(long, num_args = 1..)]
    inputs: Option<Vec<String>>,

    /// Pipe stdin to both programs instead of using args
    #[arg(long)]
    stdin: bool,

    /// Also compare stderr (default: only stdout + exit code)
    #[arg(long)]
    stderr: bool,

    /// Suppress diff output, only show pass/fail summary
    #[arg(long, short)]
    quiet: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Header
    eprintln!(
        "\n\x1b[1m\x1b[35mdifftest\x1b[0m  comparing two programs\n"
    );
    eprintln!(
        "  \x1b[2mA (oracle):\x1b[0m    \x1b[36m{}\x1b[0m",
        cli.program_a
    );
    eprintln!(
        "  \x1b[2mB (candidate):\x1b[0m  \x1b[36m{}\x1b[0m",
        cli.program_b
    );
    eprintln!();

    // Build test cases
    let inputs = if cli.stdin {
        vec![InputSource::Stdin]
    } else if let Some(ref args) = cli.inputs {
        args.iter()
            .map(|a| InputSource::Args(shell_words(a)))
            .collect()
    } else {
        // Auto-generate smoke inputs
        vec![
            InputSource::Args(vec![]),
            InputSource::Args(vec!["hello".into()]),
            InputSource::Args(vec!["Hello, World!".into()]),
            InputSource::Args(vec!["".into()]),
            InputSource::Args(vec!["42".into()]),
            InputSource::Args(vec!["-1".into()]),
            InputSource::Args(vec!["a".into(), "b".into(), "c".into()]),
        ]
    };

    eprintln!(
        "  \x1b[2mRunning {} test case{}...\x1b[0m\n",
        inputs.len(),
        if inputs.len() == 1 { "" } else { "s" }
    );

    // Run tests
    let results: Vec<RunResult> = inputs
        .iter()
        .map(|input| runner::run_pair(&cli.program_a, &cli.program_b, input, cli.stderr))
        .collect();

    // Print results
    let mut passed = 0;
    let mut failed = 0;
    let mut errored = 0;

    for r in &results {
        match r {
            RunResult::Pass { label } => {
                passed += 1;
                eprintln!("  \x1b[1m\x1b[32mPASS\x1b[0m  {}", label);
            }
            RunResult::Fail {
                label,
                stdout_diff,
                stderr_diff,
                exit_a,
                exit_b,
            } => {
                failed += 1;
                eprintln!("  \x1b[1m\x1b[31mFAIL\x1b[0m  {}", label);
                if !cli.quiet {
                    if exit_a != exit_b {
                        eprintln!(
                            "        \x1b[2mexit code:\x1b[0m A={}, B={}",
                            exit_a, exit_b
                        );
                    }
                    if let Some(diff) = stdout_diff {
                        eprintln!("        \x1b[2mstdout:\x1b[0m");
                        for line in diff.lines() {
                            if line.starts_with('-') {
                                eprintln!("          \x1b[31m{}\x1b[0m", line);
                            } else if line.starts_with('+') {
                                eprintln!("          \x1b[32m{}\x1b[0m", line);
                            } else {
                                eprintln!("          {}", line);
                            }
                        }
                    }
                    if let Some(diff) = stderr_diff {
                        eprintln!("        \x1b[2mstderr:\x1b[0m");
                        for line in diff.lines() {
                            if line.starts_with('-') {
                                eprintln!("          \x1b[31m{}\x1b[0m", line);
                            } else if line.starts_with('+') {
                                eprintln!("          \x1b[32m{}\x1b[0m", line);
                            } else {
                                eprintln!("          {}", line);
                            }
                        }
                    }
                }
            }
            RunResult::Error { label, message } => {
                errored += 1;
                eprintln!("  \x1b[1m\x1b[33mERR \x1b[0m  {} — {}", label, message);
            }
        }
    }

    // Summary
    let total = passed + failed + errored;
    eprintln!();
    if failed == 0 && errored == 0 {
        eprintln!(
            "  \x1b[1m\x1b[32m✓\x1b[0m {}/{} passed — \x1b[32mprograms are behaviorally equivalent\x1b[0m",
            passed, total
        );
    } else {
        eprintln!(
            "  \x1b[1m\x1b[31m✗\x1b[0m {}/{} passed, \x1b[31m{} failed\x1b[0m{}",
            passed,
            total,
            failed,
            if errored > 0 {
                format!(", \x1b[33m{} errors\x1b[0m", errored)
            } else {
                String::new()
            }
        );
    }
    eprintln!();

    if failed > 0 || errored > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Simple word splitting on spaces (respects quotes).
fn shell_words(s: &str) -> Vec<String> {
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

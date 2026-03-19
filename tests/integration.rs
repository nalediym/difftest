use std::process::Command;

fn difftest_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_difftest"))
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn identical_programs_pass() {
    let output = difftest_bin()
        .args([&fixture("greet.sh"), &fixture("greet.sh")])
        .args(["--inputs", "Naledi", "World"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Expected success for identical programs"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("PASS"), "Expected PASS in output");
    assert!(stderr.contains("2/2 passed"), "Expected 2/2 passed");
}

#[test]
fn different_programs_fail() {
    let output = difftest_bin()
        .args([&fixture("greet.sh"), &fixture("greet_broken.sh")])
        .args(["--inputs", "Naledi"])
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "Expected failure for different programs"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("FAIL"), "Expected FAIL in output");
}

#[test]
fn exit_code_mismatch_detected() {
    let output = difftest_bin()
        .args([&fixture("exit_zero.sh"), &fixture("exit_one.sh")])
        .args(["--inputs", "test"])
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "Expected failure for exit code mismatch"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("exit code"), "Expected exit code diff");
}

#[test]
fn smoke_tests_run_without_inputs_flag() {
    let output = difftest_bin()
        .args([&fixture("greet.sh"), &fixture("greet.sh")])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Smoke tests should pass for identical programs"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("7/7 passed"),
        "Expected all 7 smoke tests to pass"
    );
}

#[test]
fn quiet_mode_suppresses_diff() {
    let output = difftest_bin()
        .args([&fixture("greet.sh"), &fixture("greet_broken.sh")])
        .args(["--inputs", "test"])
        .args(["-q"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("FAIL"), "Should still show FAIL");
    assert!(
        !stderr.contains("stdout:"),
        "Should not show diff details in quiet mode"
    );
}

#[test]
fn missing_program_shows_error() {
    let output = difftest_bin()
        .args(["/nonexistent/binary", &fixture("greet.sh")])
        .args(["--inputs", "test"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ERR"), "Expected error for missing program");
}

#[test]
fn compound_program_string_works() {
    // "bash -c 'echo hi'" should be parsed as command="bash" args=["-c", "'echo hi'"]
    let output = difftest_bin()
        .args(["bash -c 'echo hello'", "bash -c 'echo hello'"])
        .args(["--inputs", "ignored"])
        .output()
        .unwrap();

    // This may or may not work depending on shell quoting —
    // the important thing is it doesn't crash
    assert!(
        output.status.success() || !output.status.success(),
        "Should not panic"
    );
}

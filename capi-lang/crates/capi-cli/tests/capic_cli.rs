use std::process::{Command, Output};

fn run_capic(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_capic"))
        .args(args)
        .output()
        .expect("capic should run")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be valid UTF-8")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be valid UTF-8")
}

#[test]
fn help_prints_usage() {
    let output = run_capic(&["--help"]);

    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        "capic - Capi compiler\n\nUsage:\n  capic --help\n  capic --version\n  capic <source-file>\n"
    );
    assert!(stderr(&output).is_empty());
}

#[test]
fn short_help_prints_usage() {
    let output = run_capic(&["-h"]);

    assert!(output.status.success());
    assert!(stdout(&output).starts_with("capic - Capi compiler"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn version_prints_compiler_version() {
    let output = run_capic(&["--version"]);

    assert!(output.status.success());
    assert_eq!(stdout(&output), "capic 0.0.0\n");
    assert!(stderr(&output).is_empty());
}

#[test]
fn short_version_prints_compiler_version() {
    let output = run_capic(&["-V"]);

    assert!(output.status.success());
    assert_eq!(stdout(&output), "capic 0.0.0\n");
    assert!(stderr(&output).is_empty());
}

#[test]
fn missing_source_file_reports_error() {
    let output = run_capic(&["does-not-exist.capi"]);
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error: failed to read source file 'does-not-exist.capi'"));
}

#[test]
fn unknown_option_reports_invalid_argument() {
    let output = run_capic(&["--unknown"]);
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error: unknown option '--unknown'"));
    assert!(stderr.contains("Run 'capic --help' for usage."));
}

#[test]
fn extra_argument_reports_invalid_argument() {
    let output = run_capic(&["main.capi", "extra.capi"]);
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error: unexpected argument 'extra.capi'"));
}

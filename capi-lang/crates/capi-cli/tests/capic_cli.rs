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
        "capic - Capi compiler\n\nUsage:\n  capic --help\n  capic --version\n  capic --emit tokens arquivo.capi\n  capic arquivo.capi\n"
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

#[test]
fn emit_tokens_prints_token_dump() {
    let path = std::env::temp_dir().join("capic-cli-emit-tokens.cap");
    std::fs::write(&path, "let value = 1;").expect("fixture should be written");

    let output = run_capic(&["--emit", "tokens", path.to_str().unwrap()]);
    let stdout = stdout(&output);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout.contains("Keyword(Let)"));
    assert!(stdout.contains("Identifier"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn emit_tokens_returns_failure_for_lexical_error() {
    let path = std::env::temp_dir().join("capic-cli-lexical-fail.cap");
    std::fs::write(&path, "let value = $;").expect("fixture should be written");

    let output = run_capic(&["--emit", "tokens", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error[LEX0001]: invalid character in source file"));
    assert!(stderr.contains(":1:13"));
}

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
        "capic - Capi compiler\n\nUsage:\n  capic --help\n  capic --version\n  capic --emit tokens arquivo.capi\n  capic --emit ast arquivo.capi\n  capic --emit hir arquivo.capi\n  capic check arquivo.capi\n  capic arquivo.capi\n"
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

#[test]
fn emit_tokens_returns_failure_for_invalid_utf8_source() {
    let path = std::env::temp_dir().join("capic-cli-invalid-utf8.cap");
    std::fs::write(&path, [0xff, 0xfe, 0xfd]).expect("fixture should be written");

    let output = run_capic(&["--emit", "tokens", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("is not valid UTF-8"));
}

#[test]
fn emit_ast_prints_ast_dump() {
    let path = std::env::temp_dir().join("capic-cli-emit-ast.cap");
    std::fs::write(&path, "function main() { let value = 1; }").expect("fixture should be written");

    let output = run_capic(&["--emit", "ast", path.to_str().unwrap()]);
    let stdout = stdout(&output);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout.contains("CompilationUnit"));
    assert!(stdout.contains("FunctionDecl name=main"));
    assert!(stdout.contains("LocalLet name=value"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn emit_ast_returns_failure_for_syntax_error() {
    let path = std::env::temp_dir().join("capic-cli-syntax-fail.cap");
    std::fs::write(&path, "function () { let value = ; }").expect("fixture should be written");

    let output = run_capic(&["--emit", "ast", path.to_str().unwrap()]);
    let stdout = stdout(&output);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout.is_empty());
    assert!(stderr.contains("CompilationUnit"));
    assert!(stderr.contains("error[PARSE0002]: expected function name"));
    assert!(stderr.contains("error[PARSE0006]: expected expression"));
}

#[test]
fn emit_hir_prints_resolved_hir_dump() {
    let path = std::env::temp_dir().join("capic-cli-emit-hir.cap");
    std::fs::write(&path, "function main() { let value = 1; value; }")
        .expect("fixture should be written");

    let output = run_capic(&["--emit", "hir", path.to_str().unwrap()]);
    let stdout = stdout(&output);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout.contains("Unit unit0"));
    assert!(stdout.contains("Function id=0 name=main"));
    assert!(stdout.contains("Symbols"));
    assert!(stdout.contains("name=value"));
    assert!(stdout.contains("Bindings"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn emit_hir_returns_failure_for_semantic_error() {
    let path = std::env::temp_dir().join("capic-cli-semantic-fail.cap");
    std::fs::write(&path, "function main() { missing; }").expect("fixture should be written");

    let output = run_capic(&["--emit", "hir", path.to_str().unwrap()]);
    let stdout = stdout(&output);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout.contains("Unit unit0"));
    assert!(stdout.contains("Bindings"));
    assert!(stdout.contains("not_found"));
    assert!(stderr.contains("error[SEM0002]: unresolved name `missing`"));
}

#[test]
fn emit_hir_returns_failure_for_syntax_error() {
    let path = std::env::temp_dir().join("capic-cli-hir-syntax-fail.cap");
    std::fs::write(&path, "function () { let value = ; }").expect("fixture should be written");

    let output = run_capic(&["--emit", "hir", path.to_str().unwrap()]);
    let stdout = stdout(&output);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout.is_empty());
    assert!(stderr.contains("CompilationUnit"));
    assert!(stderr.contains("error[PARSE0002]: expected function name"));
    assert!(stderr.contains("error[PARSE0006]: expected expression"));
}

#[test]
fn check_accepts_type_checked_program() {
    let path = std::env::temp_dir().join("capic-cli-check-pass.cap");
    std::fs::write(
        &path,
        "function id(value : Int) : Int { return value; } function main() { let result = id(1); }",
    )
    .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr(&output).is_empty());
}

#[test]
fn check_reports_type_errors() {
    let path = std::env::temp_dir().join("capic-cli-check-fail.cap");
    std::fs::write(&path, "function main() { let value : Bool = 1; }")
        .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error[TYPE0003]"));
    assert!(stderr.contains("type mismatch"));
}

#[test]
fn check_reports_unresolved_type() {
    let path = std::env::temp_dir().join("capic-cli-check-unresolved-type.cap");
    std::fs::write(&path, "function main(value : Missing) { value; }")
        .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error[SEM0002]"));
    assert!(stderr.contains("unresolved name `Missing`"));
    assert!(stderr.contains("error[TYPE0002]"));
    assert!(stderr.contains("unknown type `Missing`"));
}

#[test]
fn check_accepts_valid_upcast() {
    let path = std::env::temp_dir().join("capic-cli-check-upcast.cap");
    std::fs::write(
        &path,
        "class Animal {} class Dog extends Animal {} function take(value : Animal) : Animal { return value; } function main(dog : Dog) { take(dog); }",
    )
    .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr(&output).is_empty());
}

#[test]
fn check_accepts_valid_generic_application() {
    let path = std::env::temp_dir().join("capic-cli-check-generics.cap");
    std::fs::write(
        &path,
        "class Box<T> {} function main(value : Box<Int>) { value; }",
    )
    .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);

    let _ = std::fs::remove_file(path);
    assert!(output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr(&output).is_empty());
}

#[test]
fn check_reports_type_errors_deterministically() {
    let path = std::env::temp_dir().join("capic-cli-check-deterministic.cap");
    std::fs::write(&path, "function main() : Bool { return 1; }")
        .expect("fixture should be written");

    let first = run_capic(&["check", path.to_str().unwrap()]);
    let second = run_capic(&["check", path.to_str().unwrap()]);

    let _ = std::fs::remove_file(path);
    assert!(!first.status.success());
    assert!(!second.status.success());
    assert_eq!(stdout(&first), stdout(&second));
    assert_eq!(stderr(&first), stderr(&second));
}

#[test]
fn check_reports_call_without_applicable_candidate() {
    let path = std::env::temp_dir().join("capic-cli-check-non-callable.cap");
    std::fs::write(&path, "function main() { let value = 1; value(); }")
        .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error[TYPE0005]"));
    assert!(stderr.contains("callee has no callable signature"));
}

#[test]
fn check_reports_invalid_generic_application() {
    let path = std::env::temp_dir().join("capic-cli-check-invalid-generics.cap");
    std::fs::write(
        &path,
        "class Box<T> {} function main(value : Box<Int, Bool>) { value; }",
    )
    .expect("fixture should be written");

    let output = run_capic(&["check", path.to_str().unwrap()]);
    let stderr = stderr(&output);

    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr.contains("error[TYPE0008]"));
    assert!(stderr.contains("invalid generic arity"));
}

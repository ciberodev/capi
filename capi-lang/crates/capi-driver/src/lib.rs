//! Driver orchestration for the Capi compiler.

use std::path::PathBuf;

use capi_ast::dump_ast;
use capi_common::{ExitStatus, CAPI_VERSION};
use capi_diagnostics::{Diagnostic, DiagnosticBag, DiagnosticCode, DiagnosticRenderer};
use capi_lexer::{lex, Token, TokenKind};
use capi_parser::parse;
use capi_session::{CompilationSession, SessionOptions};
use capi_source::SourceMap;

/// Request accepted by the compiler driver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriverRequest {
    /// Print CLI help.
    Help,
    /// Print compiler version.
    Version,
    /// Report a structured internal compiler error.
    InternalError { message: String },
    /// Report invalid command-line arguments.
    InvalidArguments { message: String },
    /// Check that a source file can be loaded.
    CheckSource { path: PathBuf },
    /// Emit tokens for a source file.
    EmitTokens { path: PathBuf },
    /// Emit AST for a source file.
    EmitAst { path: PathBuf },
}

/// Structured response produced by the driver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriverResponse {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

impl DriverResponse {
    /// Creates a successful response.
    pub fn success(stdout: impl Into<String>) -> Self {
        Self {
            status: ExitStatus::Success,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }

    /// Creates a failed response.
    pub fn failure(stderr: impl Into<String>) -> Self {
        Self {
            status: ExitStatus::Failure,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }

    /// Creates an internal compiler error response.
    pub fn internal_error(stderr: impl Into<String>) -> Self {
        Self {
            status: ExitStatus::InternalError,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }

    /// Returns the status.
    pub const fn status(&self) -> ExitStatus {
        self.status
    }

    /// Returns stdout text.
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns stderr text.
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

/// Executes a driver request.
pub fn run(request: DriverRequest) -> DriverResponse {
    match request {
        DriverRequest::Help => DriverResponse::success(help_text()),
        DriverRequest::Version => DriverResponse::success(format!("capic {CAPI_VERSION}\n")),
        DriverRequest::InternalError { message } => {
            let mut session = initialize_session();
            session
                .diagnostics_mut()
                .push(Diagnostic::internal_error(message.clone()));
            DriverResponse::internal_error(format!("internal compiler error: {message}\n"))
        }
        DriverRequest::InvalidArguments { message } => DriverResponse::failure(format!(
            "error: {message}\n\nRun 'capic --help' for usage.\n"
        )),
        DriverRequest::CheckSource { path } => {
            let mut session = initialize_session();
            match session.sources_mut().load_file(&path) {
                Ok(_) => DriverResponse::success(String::new()),
                Err(error) => {
                    session
                        .diagnostics_mut()
                        .push(Diagnostic::error(error.to_string()));
                    DriverResponse::failure(format!("error: {error}\n"))
                }
            }
        }
        DriverRequest::EmitTokens { path } => emit_tokens(path),
        DriverRequest::EmitAst { path } => emit_ast(path),
    }
}

/// Initializes the compilation session for one Stage 0 compiler invocation.
pub fn initialize_session() -> CompilationSession {
    CompilationSession::new(SessionOptions::new(CAPI_VERSION))
}

fn help_text() -> &'static str {
    "capic - Capi compiler\n\nUsage:\n  capic --help\n  capic --version\n  capic --emit tokens arquivo.capi\n  capic --emit ast arquivo.capi\n  capic arquivo.capi\n"
}

fn emit_tokens(path: PathBuf) -> DriverResponse {
    let mut session = initialize_session();
    let source = match session.sources_mut().load_file(&path) {
        Ok(source) => source,
        Err(error) => {
            let diagnostic =
                Diagnostic::error(error.to_string()).with_code(DiagnosticCode::source(1));
            session.diagnostics_mut().push(diagnostic);
            return DriverResponse::failure(format!("error: {error}\n"));
        }
    };

    let Some(file) = session.sources().get(source) else {
        return DriverResponse::internal_error(
            "internal compiler error: loaded source is missing\n",
        );
    };
    let output = lex(source, file.text());
    let (tokens, diagnostics) = output.into_parts();
    session.diagnostics_mut().extend(diagnostics);

    let stderr = render_diagnostics(session.diagnostics(), session.sources());
    let stdout = dump_tokens(&tokens, session.sources());

    if session.diagnostics().has_internal_errors() {
        DriverResponse::internal_error(stderr)
    } else if session.diagnostics().has_errors() {
        DriverResponse::failure(stderr)
    } else {
        DriverResponse::success(stdout)
    }
}

fn emit_ast(path: PathBuf) -> DriverResponse {
    let mut session = initialize_session();
    let source = match session.sources_mut().load_file(&path) {
        Ok(source) => source,
        Err(error) => {
            let diagnostic =
                Diagnostic::error(error.to_string()).with_code(DiagnosticCode::source(1));
            session.diagnostics_mut().push(diagnostic);
            return DriverResponse::failure(format!("error: {error}\n"));
        }
    };

    let Some(file) = session.sources().get(source) else {
        return DriverResponse::internal_error(
            "internal compiler error: loaded source is missing\n",
        );
    };
    let output = lex(source, file.text());
    let (tokens, diagnostics) = output.into_parts();
    session.diagnostics_mut().extend(diagnostics);

    if !session.diagnostics().has_errors() && !session.diagnostics().has_internal_errors() {
        let parsed = parse(source, &tokens, session.sources());
        let (ast, diagnostics) = parsed.into_parts();
        session.diagnostics_mut().extend(diagnostics);
        let stderr = render_diagnostics(session.diagnostics(), session.sources());
        let stdout = dump_ast(&ast, session.sources());

        if session.diagnostics().has_internal_errors() {
            DriverResponse::internal_error(stderr)
        } else if session.diagnostics().has_errors() {
            DriverResponse::failure(format!("{stdout}{stderr}"))
        } else {
            DriverResponse::success(stdout)
        }
    } else {
        let stderr = render_diagnostics(session.diagnostics(), session.sources());
        DriverResponse::failure(stderr)
    }
}

fn render_diagnostics(diagnostics: &DiagnosticBag, sources: &SourceMap) -> String {
    DiagnosticRenderer.render_bag(diagnostics, sources)
}

fn dump_tokens(tokens: &[Token], sources: &SourceMap) -> String {
    let mut output = String::new();

    for (index, token) in tokens.iter().enumerate() {
        let span = token.span();
        let location = sources.location(span.source(), span.start());
        let end = sources.location(span.source(), span.end());
        let lexeme = match token.kind() {
            TokenKind::Eof => String::new(),
            _ => sources
                .span_text(span)
                .map(escape_lexeme)
                .unwrap_or_else(|| "<unavailable>".to_string()),
        };

        if let (Some(start), Some(end)) = (location, end) {
            let file = sources
                .get(span.source())
                .map(|file| file.path().display().to_string())
                .unwrap_or_else(|| "<unknown>".to_string());
            if lexeme.is_empty() {
                output.push_str(&format!(
                    "{index:<4} {:?} {file}:{}:{}..{}:{}\n",
                    token.kind(),
                    start.line(),
                    start.column(),
                    end.line(),
                    end.column()
                ));
            } else {
                output.push_str(&format!(
                    "{index:<4} {:?} {file}:{}:{}..{}:{} \"{}\"\n",
                    token.kind(),
                    start.line(),
                    start.column(),
                    end.line(),
                    end.column(),
                    lexeme
                ));
            }
        }
    }

    output
}

fn escape_lexeme(lexeme: &str) -> String {
    lexeme.escape_default().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_version_response() {
        let response = run(DriverRequest::Version);

        assert_eq!(response.status(), ExitStatus::Success);
        assert!(response.stdout().starts_with("capic "));
        assert!(response.stderr().is_empty());
    }

    #[test]
    fn emits_tokens_for_source_text() {
        let temp = std::env::temp_dir().join("capi-driver-emit-tokens.cap");
        std::fs::write(&temp, "let value = 1;").expect("fixture should be written");

        let response = run(DriverRequest::EmitTokens { path: temp.clone() });

        let _ = std::fs::remove_file(temp);
        assert_eq!(response.status(), ExitStatus::Success);
        assert!(response.stdout().contains("Keyword(Let)"));
        assert!(response.stdout().contains("Identifier"));
        assert!(response.stderr().is_empty());
    }

    #[test]
    fn emits_ast_for_source_text() {
        let temp = std::env::temp_dir().join("capi-driver-emit-ast.cap");
        std::fs::write(&temp, "function main() { let value = 1 + 2 * 3; }")
            .expect("fixture should be written");

        let response = run(DriverRequest::EmitAst { path: temp.clone() });

        let _ = std::fs::remove_file(temp);
        assert_eq!(response.status(), ExitStatus::Success);
        assert!(response.stdout().contains("CompilationUnit"));
        assert!(response.stdout().contains("FunctionDecl name=main"));
        assert!(response.stdout().contains("BinaryExpr op=Plus"));
        assert!(response.stderr().is_empty());
    }

    #[test]
    fn emit_tokens_fails_on_lexical_error() {
        let temp = std::env::temp_dir().join("capi-driver-lexical-fail.cap");
        std::fs::write(&temp, "let value = $;").expect("fixture should be written");

        let response = run(DriverRequest::EmitTokens { path: temp.clone() });

        let _ = std::fs::remove_file(temp);
        assert_eq!(response.status(), ExitStatus::Failure);
        assert!(response.stdout().is_empty());
        assert!(response
            .stderr()
            .contains("error[LEX0001]: invalid character in source file"));
    }

    #[test]
    fn initializes_empty_compilation_session() {
        let session = initialize_session();

        assert_eq!(session.options().version(), CAPI_VERSION);
        assert!(session.sources().is_empty());
        assert!(session.diagnostics().is_empty());
    }

    #[test]
    fn reports_missing_source_file() {
        let response = run(DriverRequest::CheckSource {
            path: PathBuf::from("does-not-exist.capi"),
        });

        assert_eq!(response.status(), ExitStatus::Failure);
        assert!(response.stderr().contains("does-not-exist.capi"));
    }

    #[test]
    fn reports_internal_errors_separately() {
        let response = run(DriverRequest::InternalError {
            message: "broken compiler invariant".to_string(),
        });

        assert_eq!(response.status(), ExitStatus::InternalError);
        assert!(response.stdout().is_empty());
        assert_eq!(
            response.stderr(),
            "internal compiler error: broken compiler invariant\n"
        );
    }

    #[test]
    fn reports_invalid_arguments() {
        let response = run(DriverRequest::InvalidArguments {
            message: "unknown option '--unknown'".to_string(),
        });

        assert_eq!(response.status(), ExitStatus::Failure);
        assert!(response.stdout().is_empty());
        assert!(response.stderr().contains("unknown option '--unknown'"));
    }
}

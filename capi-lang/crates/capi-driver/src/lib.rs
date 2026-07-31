//! Driver orchestration for the Capi compiler.

use std::path::PathBuf;

use capi_common::{ExitStatus, CAPI_VERSION};
use capi_diagnostics::Diagnostic;
use capi_session::{CompilationSession, SessionOptions};

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
    }
}

/// Initializes the compilation session for one Stage 0 compiler invocation.
pub fn initialize_session() -> CompilationSession {
    CompilationSession::new(SessionOptions::new(CAPI_VERSION))
}

fn help_text() -> &'static str {
    "capic - Capi compiler\n\nUsage:\n  capic --help\n  capic --version\n  capic <source-file>\n"
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

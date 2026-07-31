//! Structured diagnostics for the Capi compiler.

/// Severity associated with a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// A compilation error or controlled user-facing failure.
    Error,
    /// A compiler failure caused by an internal invariant violation.
    InternalError,
    /// A non-fatal warning.
    Warning,
    /// Additional contextual information.
    Note,
}

/// A structured diagnostic message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            severity,
            message: message.into(),
        }
    }

    /// Creates an error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Severity::Error, message)
    }

    /// Creates an internal compiler error diagnostic.
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(Severity::InternalError, message)
    }

    /// Returns the diagnostic severity.
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Collection of diagnostics produced during one compiler operation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    /// Adds a diagnostic to the collection.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns true when at least one error diagnostic exists.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.severity(),
                Severity::Error | Severity::InternalError
            )
        })
    }

    /// Returns true when at least one internal compiler error exists.
    pub fn has_internal_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == Severity::InternalError)
    }

    /// Iterates over collected diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Returns the number of collected diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true when no diagnostics were collected.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_error_diagnostics() {
        let mut diagnostics = DiagnosticBag::default();
        assert!(!diagnostics.has_errors());

        diagnostics.push(Diagnostic::error("missing input file"));

        assert!(diagnostics.has_errors());
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn distinguishes_internal_error_diagnostics() {
        let mut diagnostics = DiagnosticBag::default();

        diagnostics.push(Diagnostic::internal_error("broken compiler invariant"));

        assert!(diagnostics.has_errors());
        assert!(diagnostics.has_internal_errors());
    }
}

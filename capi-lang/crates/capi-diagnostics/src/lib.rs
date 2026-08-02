//! Structured diagnostics for the Capi compiler.

use std::fmt;

use capi_source::{SourceMap, Span};

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
    /// User-facing guidance for a possible fix.
    Help,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::InternalError => "internal error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }
}

/// Stable diagnostic code.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Creates a diagnostic code.
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    /// Creates a lexer diagnostic code.
    pub fn lex(number: u32) -> Self {
        Self(format!("LEX{number:04}"))
    }

    /// Creates a source diagnostic code.
    pub fn source(number: u32) -> Self {
        Self(format!("SRC{number:04}"))
    }

    /// Creates an internal compiler error code.
    pub fn internal(number: u32) -> Self {
        Self(format!("ICE{number:04}"))
    }

    /// Returns the code text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Label style in source diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelStyle {
    /// Primary error location.
    Primary,
    /// Secondary context location.
    Secondary,
}

/// A span label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticLabel {
    span: Span,
    message: Option<String>,
    style: LabelStyle,
}

impl DiagnosticLabel {
    /// Creates a primary label.
    pub fn primary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: Some(message.into()),
            style: LabelStyle::Primary,
        }
    }

    /// Creates a secondary label.
    pub fn secondary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: Some(message.into()),
            style: LabelStyle::Secondary,
        }
    }

    /// Returns the label span.
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the label message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the label style.
    pub const fn style(&self) -> LabelStyle {
        self.style
    }
}

/// Applicability of a diagnostic suggestion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Applicability {
    /// The suggestion can be applied mechanically.
    MachineApplicable,
    /// The suggestion may be incorrect.
    MaybeIncorrect,
    /// The suggestion includes placeholders.
    HasPlaceholders,
    /// Applicability has not been classified.
    Unspecified,
}

/// A structured diagnostic suggestion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticSuggestion {
    span: Span,
    replacement: String,
    applicability: Applicability,
    message: String,
}

impl DiagnosticSuggestion {
    /// Creates a suggestion.
    pub fn new(
        span: Span,
        replacement: impl Into<String>,
        applicability: Applicability,
        message: impl Into<String>,
    ) -> Self {
        Self {
            span,
            replacement: replacement.into(),
            applicability,
            message: message.into(),
        }
    }

    /// Returns the replacement span.
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the replacement text.
    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    /// Returns applicability.
    pub const fn applicability(&self) -> Applicability {
        self.applicability
    }

    /// Returns the suggestion message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// A structured diagnostic message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    severity: Severity,
    code: Option<DiagnosticCode>,
    message: String,
    primary_span: Option<Span>,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<String>,
    suggestions: Vec<DiagnosticSuggestion>,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            severity,
            code: None,
            message: message.into(),
            primary_span: None,
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
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

    /// Adds a diagnostic code.
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    /// Adds a primary span.
    pub fn with_primary_span(mut self, span: Span) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Adds a label.
    pub fn with_label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Adds a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Adds a suggestion.
    pub fn with_suggestion(mut self, suggestion: DiagnosticSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Returns the diagnostic severity.
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the diagnostic code.
    pub fn code(&self) -> Option<&DiagnosticCode> {
        self.code.as_ref()
    }

    /// Returns the diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the primary span.
    pub const fn primary_span(&self) -> Option<Span> {
        self.primary_span
    }

    /// Returns labels.
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Returns notes.
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns suggestions.
    pub fn suggestions(&self) -> &[DiagnosticSuggestion] {
        &self.suggestions
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

    /// Adds all diagnostics from an iterator.
    pub fn extend(&mut self, diagnostics: impl IntoIterator<Item = Diagnostic>) {
        self.diagnostics.extend(diagnostics);
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

    /// Consumes and returns diagnostics.
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
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

/// Renders diagnostics in the initial human-readable format.
#[derive(Clone, Copy, Debug, Default)]
pub struct DiagnosticRenderer;

impl DiagnosticRenderer {
    /// Renders a diagnostic.
    pub fn render(&self, diagnostic: &Diagnostic, sources: &SourceMap) -> String {
        let mut output = String::new();
        output.push_str(diagnostic.severity().as_str());
        if let Some(code) = diagnostic.code() {
            output.push('[');
            output.push_str(code.as_str());
            output.push(']');
        }
        output.push_str(": ");
        output.push_str(diagnostic.message());
        output.push('\n');

        if let Some(span) = diagnostic.primary_span() {
            if let Some(resolved) = sources.resolve_span(span) {
                if let Some(file) = sources.get(span.source()) {
                    let start = resolved.start();
                    output.push_str(&format!(
                        "  --> {}:{}:{}\n",
                        file.path().display(),
                        start.line(),
                        start.column()
                    ));
                    if let Some(line) = sources.line_text(span.source(), start.line()) {
                        output.push_str("   |\n");
                        output.push_str(&format!("{:>2} | {line}\n", start.line()));
                    }
                }
            }
        }

        for label in diagnostic.labels() {
            if let Some(message) = label.message() {
                let prefix = match label.style() {
                    LabelStyle::Primary => "   = ",
                    LabelStyle::Secondary => "   = note: ",
                };
                output.push_str(prefix);
                output.push_str(message);
                output.push('\n');
            }
        }

        for note in diagnostic.notes() {
            output.push_str("   = note: ");
            output.push_str(note);
            output.push('\n');
        }

        for suggestion in diagnostic.suggestions() {
            output.push_str("   = help: ");
            output.push_str(suggestion.message());
            output.push('\n');
        }

        output
    }

    /// Renders all diagnostics from a bag.
    pub fn render_bag(&self, diagnostics: &DiagnosticBag, sources: &SourceMap) -> String {
        diagnostics
            .iter()
            .map(|diagnostic| self.render(diagnostic, sources))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use capi_source::{ByteOffset, SourceMap, Span};

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

    #[test]
    fn stores_code_and_primary_span() {
        let source = capi_source::SourceId::from_raw(0);
        let span = Span::new(source, ByteOffset::from_raw(0), ByteOffset::from_raw(1)).unwrap();
        let diagnostic = Diagnostic::error("invalid character")
            .with_code(DiagnosticCode::lex(1))
            .with_primary_span(span);

        assert_eq!(
            diagnostic.code().map(DiagnosticCode::as_str),
            Some("LEX0001")
        );
        assert_eq!(diagnostic.primary_span(), Some(span));
    }

    #[test]
    fn renders_diagnostic_with_source_location() {
        let mut sources = SourceMap::default();
        let source = sources.add_file("main.cap", "$");
        let span = Span::new(source, ByteOffset::from_raw(0), ByteOffset::from_raw(1)).unwrap();
        let diagnostic = Diagnostic::error("invalid character in source file")
            .with_code(DiagnosticCode::lex(1))
            .with_primary_span(span)
            .with_label(DiagnosticLabel::primary(span, "invalid character"));

        let rendered = DiagnosticRenderer.render(&diagnostic, &sources);

        assert!(rendered.contains("error[LEX0001]: invalid character in source file"));
        assert!(rendered.contains("--> main.cap:1:1"));
        assert!(rendered.contains("invalid character"));
    }
}

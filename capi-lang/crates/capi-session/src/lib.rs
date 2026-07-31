//! Compilation session state for the Capi compiler.

use capi_diagnostics::DiagnosticBag;
use capi_source::SourceMap;

/// Options normalized for one compiler invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionOptions {
    version: String,
}

impl SessionOptions {
    /// Creates session options.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    /// Returns the compiler version associated with the session.
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// Context for one compiler invocation.
#[derive(Debug)]
pub struct CompilationSession {
    options: SessionOptions,
    sources: SourceMap,
    diagnostics: DiagnosticBag,
}

impl CompilationSession {
    /// Creates a new compilation session.
    pub fn new(options: SessionOptions) -> Self {
        Self {
            options,
            sources: SourceMap::default(),
            diagnostics: DiagnosticBag::default(),
        }
    }

    /// Returns session options.
    pub fn options(&self) -> &SessionOptions {
        &self.options
    }

    /// Returns the source map.
    pub fn sources(&self) -> &SourceMap {
        &self.sources
    }

    /// Returns the mutable source map.
    pub fn sources_mut(&mut self) -> &mut SourceMap {
        &mut self.sources
    }

    /// Returns collected diagnostics.
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }

    /// Returns mutable diagnostics.
    pub fn diagnostics_mut(&mut self) -> &mut DiagnosticBag {
        &mut self.diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_empty_session() {
        let session = CompilationSession::new(SessionOptions::new("0.0.0"));

        assert_eq!(session.options().version(), "0.0.0");
        assert!(session.sources().is_empty());
        assert!(session.diagnostics().is_empty());
    }
}

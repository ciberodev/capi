//! Source file storage and lookup for the Capi compiler.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Identifier for a source file loaded into a source map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId(u32);

impl SourceId {
    /// Creates a source id from a raw index.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw source id value.
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// A loaded source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
    id: SourceId,
    path: PathBuf,
    text: String,
}

impl SourceFile {
    /// Creates a source file from explicit parts.
    pub fn new(id: SourceId, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        Self {
            id,
            path: path.into(),
            text: text.into(),
        }
    }

    /// Returns the source id.
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the source path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the source text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Errors produced by source loading.
#[derive(Debug)]
pub struct SourceError {
    path: PathBuf,
    source: std::io::Error,
}

impl SourceError {
    /// Returns the path that failed to load.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the underlying IO error.
    pub fn source(&self) -> &std::io::Error {
        &self.source
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to read source file '{}': {}",
            self.path.display(),
            self.source
        )
    }
}

impl std::error::Error for SourceError {}

/// Storage for source files loaded during one compilation session.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    /// Adds a source file from explicit text.
    pub fn add_file(&mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> SourceId {
        let id = SourceId::from_raw(self.files.len() as u32);
        self.files.push(SourceFile::new(id, path, text));
        id
    }

    /// Loads a source file from the filesystem.
    pub fn load_file(&mut self, path: impl Into<PathBuf>) -> Result<SourceId, SourceError> {
        let path = path.into();
        let text = fs::read_to_string(&path).map_err(|source| SourceError {
            path: path.clone(),
            source,
        })?;
        Ok(self.add_file(path, text))
    }

    /// Returns a source file by id.
    pub fn get(&self, id: SourceId) -> Option<&SourceFile> {
        self.files.get(id.raw() as usize)
    }

    /// Returns the number of loaded source files.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns true when no source files were loaded.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_source_file_with_stable_id() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.capi", "");
        let file = sources.get(id).expect("source file should exist");

        assert_eq!(id.raw(), 0);
        assert_eq!(file.path(), Path::new("main.capi"));
        assert_eq!(file.text(), "");
    }
}

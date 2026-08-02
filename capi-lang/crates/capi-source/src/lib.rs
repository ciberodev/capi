//! Source file storage, spans, and location lookup for the Capi compiler.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

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

/// A byte offset in a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteOffset(u32);

impl ByteOffset {
    /// Creates a byte offset from a raw byte index.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw byte offset value.
    pub const fn raw(self) -> u32 {
        self.0
    }

    fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// A half-open source range inside a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    source: SourceId,
    start: ByteOffset,
    end: ByteOffset,
}

impl Span {
    /// Creates a span when its start is not after its end.
    pub fn new(source: SourceId, start: ByteOffset, end: ByteOffset) -> Option<Self> {
        (start <= end).then_some(Self { source, start, end })
    }

    /// Creates a span without checking its ordering.
    pub const fn new_unchecked(source: SourceId, start: ByteOffset, end: ByteOffset) -> Self {
        Self { source, start, end }
    }

    /// Returns the source id.
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the start byte offset.
    pub const fn start(self) -> ByteOffset {
        self.start
    }

    /// Returns the end byte offset.
    pub const fn end(self) -> ByteOffset {
        self.end
    }

    /// Returns true when this span covers no bytes.
    pub const fn is_empty(self) -> bool {
        self.start.raw() == self.end.raw()
    }

    /// Merges two spans from the same source.
    pub fn merge(self, other: Self) -> Option<Self> {
        if self.source != other.source {
            return None;
        }

        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self::new(self.source, start, end)
    }
}

/// A user-facing source location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    source: SourceId,
    line: u32,
    column: u32,
}

impl SourceLocation {
    /// Creates a source location from one-based line and column values.
    pub const fn new(source: SourceId, line: u32, column: u32) -> Self {
        Self {
            source,
            line,
            column,
        }
    }

    /// Returns the source id.
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the one-based line.
    pub const fn line(self) -> u32 {
        self.line
    }

    /// Returns the one-based column.
    pub const fn column(self) -> u32 {
        self.column
    }
}

/// A resolved span with display locations and source text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedSpan<'a> {
    span: Span,
    start: SourceLocation,
    end: SourceLocation,
    text: Option<&'a str>,
}

impl<'a> ResolvedSpan<'a> {
    /// Returns the original span.
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the start location.
    pub const fn start(&self) -> SourceLocation {
        self.start
    }

    /// Returns the end location.
    pub const fn end(&self) -> SourceLocation {
        self.end
    }

    /// Returns the source text covered by the span, if available.
    pub const fn text(&self) -> Option<&'a str> {
        self.text
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LineIndex {
    line_starts: Vec<ByteOffset>,
}

impl LineIndex {
    fn new(text: &str) -> Self {
        let mut line_starts = vec![ByteOffset::from_raw(0)];
        let bytes = text.as_bytes();
        let mut index = 0;

        while index < bytes.len() {
            match bytes[index] {
                b'\n' => {
                    line_starts.push(ByteOffset::from_raw((index + 1) as u32));
                    index += 1;
                }
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    line_starts.push(ByteOffset::from_raw((index + 2) as u32));
                    index += 2;
                }
                b'\r' => {
                    line_starts.push(ByteOffset::from_raw((index + 1) as u32));
                    index += 1;
                }
                _ => {
                    index += 1;
                }
            }
        }

        Self { line_starts }
    }

    fn location(&self, source: SourceId, text: &str, offset: ByteOffset) -> Option<SourceLocation> {
        let raw = offset.as_usize();
        if raw > text.len() || !text.is_char_boundary(raw) {
            return None;
        }

        let line_index = match self
            .line_starts
            .binary_search_by_key(&offset.raw(), |offset| offset.raw())
        {
            Ok(index) => index,
            Err(0) => return None,
            Err(index) => index - 1,
        };
        let line_start = self.line_starts[line_index].as_usize();
        let column = text[line_start..raw].chars().count() as u32 + 1;

        Some(SourceLocation::new(source, line_index as u32 + 1, column))
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

/// Error kind produced by source loading.
#[derive(Debug)]
pub enum SourceErrorKind {
    /// The operating system rejected the read.
    Io(std::io::Error),
    /// The source file is not valid UTF-8.
    InvalidUtf8(Utf8Error),
}

/// Errors produced by source loading.
#[derive(Debug)]
pub struct SourceError {
    path: PathBuf,
    kind: SourceErrorKind,
}

impl SourceError {
    /// Creates an I/O source error.
    pub fn io(path: PathBuf, source: std::io::Error) -> Self {
        Self {
            path,
            kind: SourceErrorKind::Io(source),
        }
    }

    /// Creates a UTF-8 source error.
    pub fn invalid_utf8(path: PathBuf, source: Utf8Error) -> Self {
        Self {
            path,
            kind: SourceErrorKind::InvalidUtf8(source),
        }
    }

    /// Returns the path that failed to load.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the source error kind.
    pub const fn kind(&self) -> &SourceErrorKind {
        &self.kind
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            SourceErrorKind::Io(source) => write!(
                formatter,
                "failed to read source file '{}': {}",
                self.path.display(),
                source
            ),
            SourceErrorKind::InvalidUtf8(source) => write!(
                formatter,
                "source file '{}' is not valid UTF-8: {}",
                self.path.display(),
                source
            ),
        }
    }
}

impl std::error::Error for SourceError {}

/// Storage for source files loaded during one compilation session.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
    line_indexes: Vec<LineIndex>,
}

impl SourceMap {
    /// Adds a source file from explicit text.
    pub fn add_file(&mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> SourceId {
        let id = SourceId::from_raw(self.files.len() as u32);
        let file = SourceFile::new(id, path, text);
        let line_index = LineIndex::new(file.text());

        self.files.push(file);
        self.line_indexes.push(line_index);

        id
    }

    /// Loads a source file from the filesystem.
    pub fn load_file(&mut self, path: impl Into<PathBuf>) -> Result<SourceId, SourceError> {
        let path = path.into();
        let bytes = fs::read(&path).map_err(|source| SourceError::io(path.clone(), source))?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|source| SourceError::invalid_utf8(path.clone(), source))?;
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

    /// Resolves a source offset to a one-based line and column.
    pub fn location(&self, source: SourceId, offset: ByteOffset) -> Option<SourceLocation> {
        let file = self.get(source)?;
        let line_index = self.line_indexes.get(source.raw() as usize)?;
        line_index.location(source, file.text(), offset)
    }

    /// Returns a source slice for a valid byte range.
    pub fn slice(&self, source: SourceId, start: ByteOffset, end: ByteOffset) -> Option<&str> {
        if start > end {
            return None;
        }

        let text = self.get(source)?.text();
        let start = start.as_usize();
        let end = end.as_usize();

        if end > text.len() || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
            return None;
        }

        Some(&text[start..end])
    }

    /// Returns the text covered by a span.
    pub fn span_text(&self, span: Span) -> Option<&str> {
        self.slice(span.source(), span.start(), span.end())
    }

    /// Resolves a span into display locations and source text.
    pub fn resolve_span(&self, span: Span) -> Option<ResolvedSpan<'_>> {
        let start = self.location(span.source(), span.start())?;
        let end = self.location(span.source(), span.end())?;
        let text = self.span_text(span);

        Some(ResolvedSpan {
            span,
            start,
            end,
            text,
        })
    }

    /// Returns a one-based line of source text without its line ending.
    pub fn line_text(&self, source: SourceId, line: u32) -> Option<&str> {
        let file = self.get(source)?;
        let line_index = self.line_indexes.get(source.raw() as usize)?;
        let index = line.checked_sub(1)? as usize;
        let start = line_index.line_starts.get(index)?.as_usize();
        let end = line_index
            .line_starts
            .get(index + 1)
            .map_or_else(|| file.text().len(), |offset| offset.as_usize());
        Some(file.text()[start..end].trim_end_matches(['\r', '\n']))
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

    #[test]
    fn loads_valid_utf8_file_from_disk() {
        let path = std::env::temp_dir().join("capi-source-valid-file.cap");
        std::fs::write(&path, "let café = \"ok\";\n").expect("valid fixture should be written");

        let mut sources = SourceMap::default();
        let id = sources.load_file(&path).expect("valid source should load");
        let file = sources.get(id).expect("loaded source should exist");

        let _ = std::fs::remove_file(&path);
        assert_eq!(id.raw(), 0);
        assert_eq!(file.path(), path.as_path());
        assert_eq!(file.text(), "let café = \"ok\";\n");
        assert_eq!(
            sources.location(id, ByteOffset::from_raw(4)),
            Some(SourceLocation::new(id, 1, 5))
        );
        assert_eq!(sources.line_text(id, 1), Some("let café = \"ok\";"));
    }

    #[test]
    fn invalid_utf8_file_returns_error_without_panic() {
        let path = std::env::temp_dir().join("capi-source-invalid-utf8.cap");
        std::fs::write(&path, [0xff, b'a', b'b']).expect("invalid fixture should be written");

        let result = std::panic::catch_unwind(|| {
            let mut sources = SourceMap::default();
            sources.load_file(&path)
        });

        let _ = std::fs::remove_file(&path);
        let error = result
            .expect("loading invalid UTF-8 should not panic")
            .expect_err("invalid UTF-8 should be rejected");
        assert!(matches!(error.kind(), SourceErrorKind::InvalidUtf8(_)));
    }

    #[test]
    fn assigns_distinct_ids() {
        let mut sources = SourceMap::default();
        let first = sources.add_file("a.cap", "");
        let second = sources.add_file("b.cap", "");

        assert_ne!(first, second);
        assert_eq!(sources.len(), 2);
    }

    #[test]
    fn resolves_locations() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "let x = 1;\nlet y = 2;");

        assert_eq!(
            sources.location(id, ByteOffset::from_raw(11)),
            Some(SourceLocation::new(id, 2, 1))
        );
    }

    #[test]
    fn resolves_locations_with_unicode_columns() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "aé\nçb");

        assert_eq!(
            sources.location(id, ByteOffset::from_raw(3)),
            Some(SourceLocation::new(id, 1, 3))
        );
        assert_eq!(
            sources.location(id, ByteOffset::from_raw(6)),
            Some(SourceLocation::new(id, 2, 2))
        );
    }

    #[test]
    fn treats_crlf_as_one_line_break() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "a\r\nb");

        assert_eq!(
            sources.location(id, ByteOffset::from_raw(3)),
            Some(SourceLocation::new(id, 2, 1))
        );
    }

    #[test]
    fn returns_line_text_without_line_ending() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "first\r\nsecond\nthird");

        assert_eq!(sources.line_text(id, 1), Some("first"));
        assert_eq!(sources.line_text(id, 2), Some("second"));
        assert_eq!(sources.line_text(id, 3), Some("third"));
        assert_eq!(sources.line_text(id, 4), None);
    }

    #[test]
    fn slices_valid_utf8_boundaries() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "aéb");

        assert_eq!(
            sources.slice(id, ByteOffset::from_raw(1), ByteOffset::from_raw(3)),
            Some("é")
        );
        assert_eq!(
            sources.slice(id, ByteOffset::from_raw(2), ByteOffset::from_raw(3)),
            None
        );
    }

    #[test]
    fn resolves_span_text_and_locations() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "let café = 1;");
        let span = Span::new(id, ByteOffset::from_raw(4), ByteOffset::from_raw(9)).unwrap();
        let resolved = sources.resolve_span(span).expect("span should resolve");

        assert_eq!(resolved.span(), span);
        assert_eq!(resolved.start(), SourceLocation::new(id, 1, 5));
        assert_eq!(resolved.end(), SourceLocation::new(id, 1, 9));
        assert_eq!(resolved.text(), Some("café"));
    }

    #[test]
    fn rejects_invalid_spans_and_locations() {
        let mut sources = SourceMap::default();
        let id = sources.add_file("main.cap", "aéb");
        let missing = SourceId::from_raw(99);

        assert!(Span::new(id, ByteOffset::from_raw(3), ByteOffset::from_raw(1)).is_none());
        assert!(sources.location(id, ByteOffset::from_raw(2)).is_none());
        assert!(sources.location(missing, ByteOffset::from_raw(0)).is_none());
        assert!(sources
            .slice(id, ByteOffset::from_raw(3), ByteOffset::from_raw(1))
            .is_none());
    }

    #[test]
    fn merges_spans_from_same_source() {
        let source = SourceId::from_raw(0);
        let left = Span::new(source, ByteOffset::from_raw(3), ByteOffset::from_raw(5)).unwrap();
        let right = Span::new(source, ByteOffset::from_raw(1), ByteOffset::from_raw(4)).unwrap();

        assert_eq!(
            left.merge(right),
            Span::new(source, ByteOffset::from_raw(1), ByteOffset::from_raw(5))
        );
    }

    #[test]
    fn does_not_merge_spans_from_different_sources() {
        let left = Span::new(
            SourceId::from_raw(0),
            ByteOffset::from_raw(1),
            ByteOffset::from_raw(2),
        )
        .unwrap();
        let right = Span::new(
            SourceId::from_raw(1),
            ByteOffset::from_raw(1),
            ByteOffset::from_raw(2),
        )
        .unwrap();

        assert_eq!(left.merge(right), None);
    }
}

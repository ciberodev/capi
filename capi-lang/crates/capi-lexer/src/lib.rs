//! Lexical analysis for the Capi compiler.

use capi_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLabel};
use capi_source::{ByteOffset, SourceId, Span};

const BOM: char = '\u{feff}';

/// A token produced by the lexer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    /// Creates a token.
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the token kind.
    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }

    /// Returns the token span.
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Token categories.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// User-defined identifier.
    Identifier,
    /// Reserved language keyword.
    Keyword(Keyword),
    /// Literal token.
    Literal(LiteralKind),
    /// Operator token.
    Operator(Operator),
    /// Delimiter token.
    Delimiter(Delimiter),
    /// End of file.
    Eof,
    /// Error recovery token.
    Error,
}

/// Capi reserved keywords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Abstract,
    Break,
    Case,
    Class,
    Const,
    Constructor,
    Continue,
    Default,
    Else,
    Extends,
    Final,
    For,
    Function,
    If,
    Implements,
    Import,
    Interface,
    Let,
    Match,
    Module,
    New,
    Override,
    Private,
    Protected,
    Public,
    Return,
    Sealed,
    Static,
    Switch,
    Trait,
    Unsafe,
    Uses,
    While,
}

/// Literal categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Integer,
    Float,
    Char,
    String,
    Bool,
}

/// Operator categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp,
    PipePipe,
    EqualEqualEqual,
    PlusPlus,
    MinusMinus,
    Arrow,
}

/// Delimiter categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Question,
    At,
}

/// Output of a lexing run.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LexOutput {
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl LexOutput {
    /// Returns tokens.
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    /// Returns diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Consumes the output into parts.
    pub fn into_parts(self) -> (Vec<Token>, Vec<Diagnostic>) {
        (self.tokens, self.diagnostics)
    }
}

/// Lexes a source string.
pub fn lex(source: SourceId, text: &str) -> LexOutput {
    Lexer::new(source, text).lex()
}

struct Lexer<'a> {
    source: SourceId,
    text: &'a str,
    offset: usize,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Lexer<'a> {
    fn new(source: SourceId, text: &'a str) -> Self {
        Self {
            source,
            text,
            offset: 0,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn lex(mut self) -> LexOutput {
        if self.peek() == Some(BOM) {
            self.bump();
        }

        while !self.is_eof() {
            let start = self.offset;

            if self.skip_whitespace() {
                continue;
            }

            if self.skip_comment() {
                continue;
            }

            if self.peek().is_some_and(is_identifier_start) {
                self.lex_identifier_or_keyword();
                continue;
            }

            if self
                .peek()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.lex_number();
                continue;
            }

            match self.peek() {
                Some('"') => self.lex_string(),
                Some('\'') => self.lex_char(),
                Some(_) => {
                    if self.lex_operator_or_delimiter() {
                        continue;
                    }
                    self.invalid_character(start);
                }
                None => break,
            }
        }

        let eof = self.empty_span(self.offset);
        self.tokens.push(Token::new(TokenKind::Eof, eof));

        LexOutput {
            tokens: self.tokens,
            diagnostics: self.diagnostics,
        }
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn peek(&self) -> Option<char> {
        self.text[self.offset..].chars().next()
    }

    fn starts_with(&self, needle: &str) -> bool {
        self.text[self.offset..].starts_with(needle)
    }

    fn bump(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.offset += character.len_utf8();
        Some(character)
    }

    fn skip_whitespace(&mut self) -> bool {
        let start = self.offset;
        while let Some(character) = self.peek() {
            if matches!(character, ' ' | '\t' | '\n' | '\r') {
                self.bump();
            } else {
                break;
            }
        }
        self.offset != start
    }

    fn skip_comment(&mut self) -> bool {
        if self.starts_with("//") {
            while let Some(character) = self.peek() {
                if matches!(character, '\n' | '\r') {
                    break;
                }
                self.bump();
            }
            return true;
        }

        if self.starts_with("/*") {
            let start = self.offset;
            self.offset += 2;
            while !self.is_eof() {
                if self.starts_with("*/") {
                    self.offset += 2;
                    return true;
                }
                self.bump();
            }
            let span = self.span(start, self.offset);
            self.diagnostics.push(
                Diagnostic::error("unterminated block comment")
                    .with_code(DiagnosticCode::lex(8))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(span, "block comment starts here"))
                    .with_note("close the block comment with `*/`"),
            );
            return true;
        }

        false
    }

    fn lex_identifier_or_keyword(&mut self) {
        let start = self.offset;
        self.bump();
        while self.peek().is_some_and(is_identifier_continue) {
            self.bump();
        }

        let lexeme = &self.text[start..self.offset];
        let kind = match lexeme {
            "true" | "false" => TokenKind::Literal(LiteralKind::Bool),
            _ => keyword(lexeme).map_or(TokenKind::Identifier, TokenKind::Keyword),
        };
        self.push(kind, start, self.offset);
    }

    fn lex_number(&mut self) {
        let start = self.offset;
        while self
            .peek()
            .is_some_and(|character| character.is_ascii_digit())
        {
            self.bump();
        }

        let mut kind = LiteralKind::Integer;
        if self.peek() == Some('.') {
            let dot = self.offset;
            self.bump();
            if self
                .peek()
                .is_some_and(|character| character.is_ascii_digit())
            {
                kind = LiteralKind::Float;
                while self
                    .peek()
                    .is_some_and(|character| character.is_ascii_digit())
                {
                    self.bump();
                }
            } else {
                self.offset = dot;
            }
        }

        if self.peek().is_some_and(is_identifier_start) {
            while self.peek().is_some_and(is_identifier_continue) {
                self.bump();
            }

            let span = self.span(start, self.offset);
            self.diagnostics.push(
                Diagnostic::error("invalid numeric literal")
                    .with_code(DiagnosticCode::lex(2))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(span, "invalid numeric literal")),
            );
            self.tokens.push(Token::new(TokenKind::Error, span));
            return;
        }

        self.push(TokenKind::Literal(kind), start, self.offset);
    }

    fn lex_string(&mut self) {
        let start = self.offset;
        self.bump();

        while let Some(character) = self.peek() {
            match character {
                '"' => {
                    self.bump();
                    self.push(TokenKind::Literal(LiteralKind::String), start, self.offset);
                    return;
                }
                '\\' => {
                    let escape_start = self.offset;
                    self.bump();
                    match self.peek() {
                        Some('\\' | '"' | 'n' | 'r' | 't') => {
                            self.bump();
                        }
                        Some(_) => {
                            self.bump();
                            let span = self.span(escape_start, self.offset);
                            self.diagnostics.push(
                                Diagnostic::error("invalid escape sequence")
                                    .with_code(DiagnosticCode::lex(7))
                                    .with_primary_span(span)
                                    .with_label(DiagnosticLabel::primary(
                                        span,
                                        "unknown escape sequence",
                                    )),
                            );
                        }
                        None => break,
                    }
                }
                '\n' | '\r' => break,
                _ => {
                    self.bump();
                }
            }
        }

        let span = self.span(start, self.offset);
        self.diagnostics.push(
            Diagnostic::error("unterminated string literal")
                .with_code(DiagnosticCode::lex(3))
                .with_primary_span(span)
                .with_label(DiagnosticLabel::primary(span, "string starts here"))
                .with_note("close the string with a double quote"),
        );
        self.push(TokenKind::Error, start, self.offset);
    }

    fn lex_char(&mut self) {
        let start = self.offset;
        self.bump();
        let mut count = 0_u32;
        let mut terminated = false;

        while let Some(character) = self.peek() {
            match character {
                '\'' => {
                    self.bump();
                    terminated = true;
                    break;
                }
                '\\' => {
                    let escape_start = self.offset;
                    self.bump();
                    match self.peek() {
                        Some('\\' | '\'' | '"' | 'n' | 'r' | 't') => {
                            self.bump();
                            count += 1;
                        }
                        Some(_) => {
                            self.bump();
                            count += 1;
                            let escape_span = self.span(escape_start, self.offset);
                            self.diagnostics.push(
                                Diagnostic::error("invalid escape sequence")
                                    .with_code(DiagnosticCode::lex(7))
                                    .with_primary_span(escape_span)
                                    .with_label(DiagnosticLabel::primary(
                                        escape_span,
                                        "unknown escape sequence",
                                    )),
                            );
                        }
                        None => break,
                    }
                }
                '\n' | '\r' => break,
                _ => {
                    self.bump();
                    count += 1;
                }
            }
        }

        let span = self.span(start, self.offset);
        if !terminated {
            self.diagnostics.push(
                Diagnostic::error("unterminated character literal")
                    .with_code(DiagnosticCode::lex(5))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(
                        span,
                        "character literal starts here",
                    ))
                    .with_note("close the character literal with a single quote"),
            );
            self.push(TokenKind::Error, start, self.offset);
        } else if count == 0 {
            self.diagnostics.push(
                Diagnostic::error("empty character literal")
                    .with_code(DiagnosticCode::lex(5))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(span, "character literal is empty")),
            );
            self.push(TokenKind::Error, start, self.offset);
        } else if count > 1 {
            self.diagnostics.push(
                Diagnostic::error("character literal contains more than one character")
                    .with_code(DiagnosticCode::lex(6))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(
                        span,
                        "character literal spans multiple characters",
                    )),
            );
            self.push(TokenKind::Error, start, self.offset);
        } else {
            self.push(TokenKind::Literal(LiteralKind::Char), start, self.offset);
        }
    }

    fn lex_operator_or_delimiter(&mut self) -> bool {
        let start = self.offset;
        let kind = if self.starts_with("===") {
            self.offset += 3;
            TokenKind::Operator(Operator::EqualEqualEqual)
        } else if self.starts_with("==") {
            self.offset += 2;
            TokenKind::Operator(Operator::EqualEqual)
        } else if self.starts_with("!=") {
            self.offset += 2;
            TokenKind::Operator(Operator::BangEqual)
        } else if self.starts_with("<=") {
            self.offset += 2;
            TokenKind::Operator(Operator::LessEqual)
        } else if self.starts_with(">=") {
            self.offset += 2;
            TokenKind::Operator(Operator::GreaterEqual)
        } else if self.starts_with("&&") {
            self.offset += 2;
            TokenKind::Operator(Operator::AmpAmp)
        } else if self.starts_with("||") {
            self.offset += 2;
            TokenKind::Operator(Operator::PipePipe)
        } else if self.starts_with("++") {
            self.offset += 2;
            TokenKind::Operator(Operator::PlusPlus)
        } else if self.starts_with("--") {
            self.offset += 2;
            TokenKind::Operator(Operator::MinusMinus)
        } else if self.starts_with("->") {
            self.offset += 2;
            TokenKind::Operator(Operator::Arrow)
        } else if let Some(character) = self.bump() {
            match character {
                '+' => TokenKind::Operator(Operator::Plus),
                '-' => TokenKind::Operator(Operator::Minus),
                '*' => TokenKind::Operator(Operator::Star),
                '/' => TokenKind::Operator(Operator::Slash),
                '%' => TokenKind::Operator(Operator::Percent),
                '=' => TokenKind::Operator(Operator::Equal),
                '!' => TokenKind::Operator(Operator::Bang),
                '<' => TokenKind::Operator(Operator::Less),
                '>' => TokenKind::Operator(Operator::Greater),
                '(' => TokenKind::Delimiter(Delimiter::LeftParen),
                ')' => TokenKind::Delimiter(Delimiter::RightParen),
                '{' => TokenKind::Delimiter(Delimiter::LeftBrace),
                '}' => TokenKind::Delimiter(Delimiter::RightBrace),
                '[' => TokenKind::Delimiter(Delimiter::LeftBracket),
                ']' => TokenKind::Delimiter(Delimiter::RightBracket),
                ',' => TokenKind::Delimiter(Delimiter::Comma),
                '.' => TokenKind::Delimiter(Delimiter::Dot),
                ';' => TokenKind::Delimiter(Delimiter::Semicolon),
                ':' => TokenKind::Delimiter(Delimiter::Colon),
                '?' => TokenKind::Delimiter(Delimiter::Question),
                '@' => TokenKind::Delimiter(Delimiter::At),
                _ => {
                    self.offset = start;
                    return false;
                }
            }
        } else {
            return false;
        };

        self.push(kind, start, self.offset);
        true
    }

    fn invalid_character(&mut self, start: usize) {
        self.bump();
        let span = self.span(start, self.offset);
        self.diagnostics.push(
            Diagnostic::error("invalid character in source file")
                .with_code(DiagnosticCode::lex(1))
                .with_primary_span(span)
                .with_label(DiagnosticLabel::primary(span, "invalid character")),
        );
        self.tokens.push(Token::new(TokenKind::Error, span));
    }

    fn push(&mut self, kind: TokenKind, start: usize, end: usize) {
        let span = self.span(start, end);
        self.tokens.push(Token::new(kind, span));
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new_unchecked(
            self.source,
            ByteOffset::from_raw(start as u32),
            ByteOffset::from_raw(end as u32),
        )
    }

    fn empty_span(&self, offset: usize) -> Span {
        self.span(offset, offset)
    }
}

fn is_identifier_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

fn is_identifier_continue(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}

fn keyword(lexeme: &str) -> Option<Keyword> {
    match lexeme {
        "abstract" => Some(Keyword::Abstract),
        "break" => Some(Keyword::Break),
        "case" => Some(Keyword::Case),
        "class" => Some(Keyword::Class),
        "const" => Some(Keyword::Const),
        "constructor" => Some(Keyword::Constructor),
        "continue" => Some(Keyword::Continue),
        "default" => Some(Keyword::Default),
        "else" => Some(Keyword::Else),
        "extends" => Some(Keyword::Extends),
        "final" => Some(Keyword::Final),
        "for" => Some(Keyword::For),
        "function" => Some(Keyword::Function),
        "if" => Some(Keyword::If),
        "implements" => Some(Keyword::Implements),
        "import" => Some(Keyword::Import),
        "interface" => Some(Keyword::Interface),
        "let" => Some(Keyword::Let),
        "match" => Some(Keyword::Match),
        "module" => Some(Keyword::Module),
        "new" => Some(Keyword::New),
        "override" => Some(Keyword::Override),
        "private" => Some(Keyword::Private),
        "protected" => Some(Keyword::Protected),
        "public" => Some(Keyword::Public),
        "return" => Some(Keyword::Return),
        "sealed" => Some(Keyword::Sealed),
        "static" => Some(Keyword::Static),
        "switch" => Some(Keyword::Switch),
        "trait" => Some(Keyword::Trait),
        "unsafe" => Some(Keyword::Unsafe),
        "uses" => Some(Keyword::Uses),
        "while" => Some(Keyword::While),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use capi_source::SourceMap;

    fn token_kinds(text: &str) -> Vec<TokenKind> {
        let source = SourceId::from_raw(0);
        lex(source, text)
            .tokens()
            .iter()
            .map(|token| token.kind().clone())
            .collect()
    }

    #[test]
    fn emits_eof_for_empty_file() {
        assert_eq!(token_kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn lexes_identifier_and_keyword() {
        assert_eq!(
            token_kinds("let letter _name name1"),
            vec![
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_literals() {
        assert_eq!(
            token_kinds("123 3.14 \"capi\" 'x' true false"),
            vec![
                TokenKind::Literal(LiteralKind::Integer),
                TokenKind::Literal(LiteralKind::Float),
                TokenKind::Literal(LiteralKind::String),
                TokenKind::Literal(LiteralKind::Char),
                TokenKind::Literal(LiteralKind::Bool),
                TokenKind::Literal(LiteralKind::Bool),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn applies_maximal_munch() {
        assert_eq!(
            token_kinds("=== == >= && ||"),
            vec![
                TokenKind::Operator(Operator::EqualEqualEqual),
                TokenKind::Operator(Operator::EqualEqual),
                TokenKind::Operator(Operator::GreaterEqual),
                TokenKind::Operator(Operator::AmpAmp),
                TokenKind::Operator(Operator::PipePipe),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn discards_comments_and_whitespace() {
        assert_eq!(
            token_kinds("let // comment\nx /* block */ = 1"),
            vec![
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Identifier,
                TokenKind::Operator(Operator::Equal),
                TokenKind::Literal(LiteralKind::Integer),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn reports_unterminated_string() {
        let output = lex(SourceId::from_raw(0), "\"abc");

        assert!(output
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message() == "unterminated string literal"));
        assert_eq!(output.tokens()[0].kind(), &TokenKind::Error);
    }

    #[test]
    fn reports_invalid_numeric_literal() {
        let output = lex(SourceId::from_raw(0), "123abc");

        assert_eq!(output.tokens()[0].kind(), &TokenKind::Error);
        assert!(output
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message() == "invalid numeric literal"));
    }

    #[test]
    fn reports_invalid_char_escape() {
        let output = lex(SourceId::from_raw(0), "'\\x'");

        assert!(output
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message() == "invalid escape sequence"));
    }

    #[test]
    fn reports_invalid_character_with_span() {
        let output = lex(SourceId::from_raw(0), "$");

        assert_eq!(output.tokens()[0].kind(), &TokenKind::Error);
        assert!(output.diagnostics()[0].primary_span().is_some());
    }

    #[test]
    fn token_span_recovers_lexeme() {
        let mut sources = SourceMap::default();
        let source = sources.add_file("main.cap", "let value = 1;");
        let output = lex(source, sources.get(source).unwrap().text());

        assert_eq!(sources.span_text(output.tokens()[0].span()), Some("let"));
    }
}

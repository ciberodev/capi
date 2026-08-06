use std::fs;
use std::path::PathBuf;

use capi_diagnostics::Severity;
use capi_lexer::{lex, Delimiter, Keyword, LiteralKind, Operator, Token, TokenKind};
use capi_source::{SourceLocation, SourceMap};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn lex_fixture(path: &str) -> (SourceMap, Vec<Token>, Vec<capi_diagnostics::Diagnostic>) {
    let text = fs::read_to_string(workspace_fixture(path)).expect("fixture should be readable");
    let mut sources = SourceMap::default();
    let source = sources.add_file(path, text);
    let output = lex(source, sources.get(source).unwrap().text());
    let (tokens, diagnostics) = output.into_parts();
    (sources, tokens, diagnostics)
}

fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
    tokens.iter().map(|token| token.kind().clone()).collect()
}

fn non_eof_kinds(tokens: &[Token]) -> Vec<TokenKind> {
    tokens
        .iter()
        .filter(|token| token.kind() != &TokenKind::Eof)
        .map(|token| token.kind().clone())
        .collect()
}

fn lex_text(path: &str, text: &str) -> (SourceMap, Vec<Token>, Vec<capi_diagnostics::Diagnostic>) {
    let mut sources = SourceMap::default();
    let source = sources.add_file(path, text);
    let output = lex(source, sources.get(source).unwrap().text());
    let (tokens, diagnostics) = output.into_parts();
    (sources, tokens, diagnostics)
}

fn token_texts<'a>(sources: &'a SourceMap, tokens: &'a [Token]) -> Vec<&'a str> {
    tokens
        .iter()
        .filter(|token| token.kind() != &TokenKind::Eof)
        .map(|token| sources.span_text(token.span()).expect("token span text"))
        .collect()
}

fn dump_tokens(tokens: &[Token], sources: &SourceMap) -> String {
    let mut output = String::new();

    for (index, token) in tokens.iter().enumerate() {
        let span = token.span();
        let start = sources.location(span.source(), span.start()).unwrap();
        let end = sources.location(span.source(), span.end()).unwrap();
        let file = sources.get(span.source()).unwrap().path().display();

        if matches!(token.kind(), TokenKind::Eof) {
            output.push_str(&format!(
                "{index:<4} {:?} {file}:{}:{}..{}:{}\n",
                token.kind(),
                start.line(),
                start.column(),
                end.line(),
                end.column()
            ));
        } else {
            let lexeme = sources
                .span_text(span)
                .unwrap()
                .escape_default()
                .to_string();
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

    output
}

fn assert_diagnostic_span(
    sources: &SourceMap,
    diagnostics: &[capi_diagnostics::Diagnostic],
    message: &str,
    start: (u32, u32),
    end: (u32, u32),
    text: &str,
) {
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.message() == message)
        .expect("diagnostic should exist");
    let span = diagnostic
        .primary_span()
        .expect("diagnostic should have a primary span");

    assert_eq!(
        sources.location(span.source(), span.start()),
        Some(SourceLocation::new(span.source(), start.0, start.1))
    );
    assert_eq!(
        sources.location(span.source(), span.end()),
        Some(SourceLocation::new(span.source(), end.0, end.1))
    );
    assert_eq!(sources.span_text(span), Some(text));
}

fn assert_structured_lexer_diagnostic(diagnostic: &capi_diagnostics::Diagnostic) {
    assert_eq!(diagnostic.severity(), Severity::Error);
    assert!(
        diagnostic
            .code()
            .is_some_and(|code| code.as_str().starts_with("LEX")),
        "lexer diagnostic should have a LEX code"
    );
    let primary_span = diagnostic
        .primary_span()
        .expect("lexer diagnostic should have a primary span");
    assert!(
        diagnostic
            .labels()
            .iter()
            .any(|label| label.span() == primary_span),
        "lexer diagnostic should have a label on the primary span"
    );
}

#[test]
fn lexes_basic_fixture() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/pass/basic.cap");

    assert!(diagnostics.is_empty());
    assert_eq!(
        kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Operator(Operator::Equal),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Delimiter(capi_lexer::Delimiter::Semicolon),
            TokenKind::Eof,
        ]
    );
}

#[test]
fn basic_dump_matches_snapshot() {
    let (sources, tokens, diagnostics) = lex_fixture("tests/lexer/pass/basic.cap");
    let snapshot = fs::read_to_string(workspace_fixture("tests/lexer/snapshots/basic.tokens.snap"))
        .expect("snapshot should be readable");

    assert!(diagnostics.is_empty());
    assert_eq!(dump_tokens(&tokens, &sources), snapshot);
}

#[test]
fn empty_file_dump_contains_only_eof_at_final_offset() {
    let (sources, tokens, diagnostics) = lex_text("empty.cap", "");

    assert!(diagnostics.is_empty());
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind(), &TokenKind::Eof);
    assert_eq!(tokens[0].span().start(), tokens[0].span().end());
    assert_eq!(tokens[0].span().start().raw(), 0);
    assert_eq!(
        dump_tokens(&tokens, &sources),
        "0    Eof empty.cap:1:1..1:1\n"
    );
}

#[test]
fn lexes_operator_fixture_with_maximal_munch() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/pass/operators.cap");

    assert!(diagnostics.is_empty());
    assert!(tokens
        .iter()
        .any(|token| token.kind() == &TokenKind::Operator(Operator::EqualEqualEqual)));
    assert!(tokens
        .iter()
        .any(|token| token.kind() == &TokenKind::Operator(Operator::GreaterEqual)));
    assert!(tokens
        .iter()
        .any(|token| token.kind() == &TokenKind::Operator(Operator::AmpAmp)));
}

#[test]
fn lexes_all_operator_forms() {
    let mut sources = SourceMap::default();
    let source = sources.add_file(
        "operators.cap",
        "+ - * / % = == ! != < <= > >= && || === ++ -- ->",
    );
    let output = lex(source, sources.get(source).unwrap().text());

    assert!(output.diagnostics().is_empty());
    assert_eq!(
        non_eof_kinds(output.tokens()),
        vec![
            TokenKind::Operator(Operator::Plus),
            TokenKind::Operator(Operator::Minus),
            TokenKind::Operator(Operator::Star),
            TokenKind::Operator(Operator::Slash),
            TokenKind::Operator(Operator::Percent),
            TokenKind::Operator(Operator::Equal),
            TokenKind::Operator(Operator::EqualEqual),
            TokenKind::Operator(Operator::Bang),
            TokenKind::Operator(Operator::BangEqual),
            TokenKind::Operator(Operator::Less),
            TokenKind::Operator(Operator::LessEqual),
            TokenKind::Operator(Operator::Greater),
            TokenKind::Operator(Operator::GreaterEqual),
            TokenKind::Operator(Operator::AmpAmp),
            TokenKind::Operator(Operator::PipePipe),
            TokenKind::Operator(Operator::EqualEqualEqual),
            TokenKind::Operator(Operator::PlusPlus),
            TokenKind::Operator(Operator::MinusMinus),
            TokenKind::Operator(Operator::Arrow),
        ]
    );
}

#[test]
fn lexes_unicode_fixture_with_byte_spans_and_character_columns() {
    let (sources, tokens, diagnostics) = lex_fixture("tests/lexer/pass/unicode.cap");

    assert!(diagnostics.is_empty());

    let string = tokens
        .iter()
        .find(|token| token.kind() == &TokenKind::Literal(LiteralKind::String))
        .expect("string token should exist");
    let character = tokens
        .iter()
        .find(|token| token.kind() == &TokenKind::Literal(LiteralKind::Char))
        .expect("char token should exist");

    assert_eq!(sources.span_text(string.span()), Some("\"Capi\""));
    assert_eq!(sources.span_text(character.span()), Some("'ç'"));
    assert_eq!(
        sources.location(character.span().source(), character.span().start()),
        Some(SourceLocation::new(character.span().source(), 2, 14))
    );
    assert_eq!(
        sources.location(character.span().source(), character.span().end()),
        Some(SourceLocation::new(character.span().source(), 2, 17))
    );
}

#[test]
fn lexes_identifier_forms() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("identifiers.cap", "_ alpha alpha1 Camel");
    let output = lex(source, sources.get(source).unwrap().text());

    assert!(output.diagnostics().is_empty());
    assert_eq!(
        non_eof_kinds(output.tokens()),
        vec![
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
        ]
    );
}

#[test]
fn lexes_keywords_case_sensitively_and_keeps_prefixes_as_identifiers() {
    let (sources, tokens, diagnostics) = lex_text(
        "keyword-prefixes.cap",
        "let letter Class class className true false trueValue false_value",
    );

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Keyword(Keyword::Class),
            TokenKind::Identifier,
            TokenKind::Literal(LiteralKind::Bool),
            TokenKind::Literal(LiteralKind::Bool),
            TokenKind::Identifier,
            TokenKind::Identifier,
        ]
    );
    assert_eq!(
        token_texts(&sources, &tokens),
        vec![
            "let",
            "letter",
            "Class",
            "class",
            "className",
            "true",
            "false",
            "trueValue",
            "false_value",
        ]
    );
}

#[test]
fn lexes_reserved_keywords() {
    let mut sources = SourceMap::default();
    let source = sources.add_file(
        "keywords.cap",
        "abstract break case class const constructor continue default else extends final for \
         function if implements import interface let match module new override private protected \
         public return sealed static switch trait unsafe uses while",
    );
    let output = lex(source, sources.get(source).unwrap().text());

    assert!(output.diagnostics().is_empty());
    assert_eq!(
        non_eof_kinds(output.tokens()),
        vec![
            TokenKind::Keyword(Keyword::Abstract),
            TokenKind::Keyword(Keyword::Break),
            TokenKind::Keyword(Keyword::Case),
            TokenKind::Keyword(Keyword::Class),
            TokenKind::Keyword(Keyword::Const),
            TokenKind::Keyword(Keyword::Constructor),
            TokenKind::Keyword(Keyword::Continue),
            TokenKind::Keyword(Keyword::Default),
            TokenKind::Keyword(Keyword::Else),
            TokenKind::Keyword(Keyword::Extends),
            TokenKind::Keyword(Keyword::Final),
            TokenKind::Keyword(Keyword::For),
            TokenKind::Keyword(Keyword::Function),
            TokenKind::Keyword(Keyword::If),
            TokenKind::Keyword(Keyword::Implements),
            TokenKind::Keyword(Keyword::Import),
            TokenKind::Keyword(Keyword::Interface),
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Keyword(Keyword::Match),
            TokenKind::Keyword(Keyword::Module),
            TokenKind::Keyword(Keyword::New),
            TokenKind::Keyword(Keyword::Override),
            TokenKind::Keyword(Keyword::Private),
            TokenKind::Keyword(Keyword::Protected),
            TokenKind::Keyword(Keyword::Public),
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Keyword(Keyword::Sealed),
            TokenKind::Keyword(Keyword::Static),
            TokenKind::Keyword(Keyword::Switch),
            TokenKind::Keyword(Keyword::Trait),
            TokenKind::Keyword(Keyword::Unsafe),
            TokenKind::Keyword(Keyword::Uses),
            TokenKind::Keyword(Keyword::While),
        ]
    );
}

#[test]
fn lexes_literal_forms() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("literals.cap", "0 42 3.14 \"text\\n\" 'x' '\\n' true false");
    let output = lex(source, sources.get(source).unwrap().text());

    assert!(output.diagnostics().is_empty());
    assert_eq!(
        non_eof_kinds(output.tokens()),
        vec![
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Literal(LiteralKind::Float),
            TokenKind::Literal(LiteralKind::String),
            TokenKind::Literal(LiteralKind::Char),
            TokenKind::Literal(LiteralKind::Char),
            TokenKind::Literal(LiteralKind::Bool),
            TokenKind::Literal(LiteralKind::Bool),
        ]
    );
}

#[test]
fn lexes_number_edge_cases() {
    let (sources, tokens, diagnostics) = lex_text("numbers.cap", "0 123; 42 \n-1 0.0 3.14 1.");

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Delimiter(Delimiter::Semicolon),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Operator(Operator::Minus),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Literal(LiteralKind::Float),
            TokenKind::Literal(LiteralKind::Float),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Delimiter(Delimiter::Dot),
        ]
    );
    assert_eq!(
        token_texts(&sources, &tokens),
        vec!["0", "123", ";", "42", "-", "1", "0.0", "3.14", "1", "."]
    );
}

#[test]
fn lexes_all_delimiters() {
    let (sources, tokens, diagnostics) = lex_text("delimiters.cap", "( ) { } [ ] , . ; : ? @");

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Delimiter(Delimiter::LeftParen),
            TokenKind::Delimiter(Delimiter::RightParen),
            TokenKind::Delimiter(Delimiter::LeftBrace),
            TokenKind::Delimiter(Delimiter::RightBrace),
            TokenKind::Delimiter(Delimiter::LeftBracket),
            TokenKind::Delimiter(Delimiter::RightBracket),
            TokenKind::Delimiter(Delimiter::Comma),
            TokenKind::Delimiter(Delimiter::Dot),
            TokenKind::Delimiter(Delimiter::Semicolon),
            TokenKind::Delimiter(Delimiter::Colon),
            TokenKind::Delimiter(Delimiter::Question),
            TokenKind::Delimiter(Delimiter::At),
        ]
    );
    for token in tokens
        .iter()
        .filter(|token| token.kind() != &TokenKind::Eof)
    {
        assert_eq!(token.span().end().raw() - token.span().start().raw(), 1);
    }
    assert_eq!(
        token_texts(&sources, &tokens),
        vec!["(", ")", "{", "}", "[", "]", ",", ".", ";", ":", "?", "@"]
    );
}

#[test]
fn discards_line_and_block_comments() {
    let (sources, tokens, diagnostics) = lex_text(
        "comments.cap",
        "let // ignored\nx /* ignored */ = 1\n// eof comment",
    );

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Operator(Operator::Equal),
            TokenKind::Literal(LiteralKind::Integer),
        ]
    );
    assert_eq!(token_texts(&sources, &tokens), vec!["let", "x", "=", "1"]);
}

#[test]
fn treats_comment_openers_as_comments_not_operators() {
    let (sources, tokens, diagnostics) =
        lex_text("comment-openers.cap", "a // + - *\nb /* == <= */ c");

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
        ]
    );
    assert_eq!(token_texts(&sources, &tokens), vec!["a", "b", "c"]);
}

#[test]
fn discards_multiline_block_comment_and_line_comment_to_eof() {
    let (sources, tokens, diagnostics) = lex_text(
        "comment-forms.cap",
        "let /* line 1\nolá\nline 3 */ value = 1; // ignored to eof",
    );

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Operator(Operator::Equal),
            TokenKind::Literal(LiteralKind::Integer),
            TokenKind::Delimiter(Delimiter::Semicolon),
        ]
    );
    assert_eq!(
        token_texts(&sources, &tokens),
        vec!["let", "value", "=", "1", ";"]
    );
}

#[test]
fn discards_block_comments_with_unicode_and_nested_openers_as_content() {
    let (sources, tokens, diagnostics) = lex_text(
        "unicode-comments.cap",
        "let /* olá /* nested opener */ value = 1",
    );

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Operator(Operator::Equal),
            TokenKind::Literal(LiteralKind::Integer),
        ]
    );
    assert_eq!(
        token_texts(&sources, &tokens),
        vec!["let", "value", "=", "1"]
    );
}

#[test]
fn token_dumps_are_deterministic_for_required_forms() {
    for (path, text, expected_fragments) in [
        (
            "dump-keywords.cap",
            "function main() { return true; }",
            vec!["Keyword(Function)", "Identifier", "Literal(Bool)"],
        ),
        (
            "dump-literals.cap",
            "let s = \"olá\"; let c = 'ç';",
            vec!["Literal(String)", "Literal(Char)"],
        ),
        (
            "dump-operators.cap",
            "a === b && c <= d",
            vec!["EqualEqualEqual", "AmpAmp", "LessEqual"],
        ),
        (
            "dump-comments.cap",
            "let /* ignored */ value = 1;",
            vec!["Keyword(Let)", "Identifier", "Literal(Integer)"],
        ),
    ] {
        let (first_sources, first_tokens, first_diagnostics) = lex_text(path, text);
        let (second_sources, second_tokens, second_diagnostics) = lex_text(path, text);
        let first_dump = dump_tokens(&first_tokens, &first_sources);
        let second_dump = dump_tokens(&second_tokens, &second_sources);

        assert!(first_diagnostics.is_empty());
        assert!(second_diagnostics.is_empty());
        assert_eq!(first_dump, second_dump);
        assert!(!first_dump.contains("0x"));
        for fragment in expected_fragments {
            assert!(
                first_dump.contains(fragment),
                "dump for {path} should contain {fragment}: {first_dump}"
            );
        }
    }
}

#[test]
fn whitespace_and_crlf_do_not_shift_token_locations() {
    let (sources, tokens, diagnostics) = lex_text("whitespace.cap", " \t\r\n  let\rvalue\n");

    assert!(diagnostics.is_empty());
    assert_eq!(
        non_eof_kinds(&tokens),
        vec![TokenKind::Keyword(Keyword::Let), TokenKind::Identifier]
    );
    assert_eq!(
        sources.location(tokens[0].span().source(), tokens[0].span().start()),
        Some(SourceLocation::new(tokens[0].span().source(), 2, 3))
    );
    assert_eq!(
        sources.location(tokens[1].span().source(), tokens[1].span().start()),
        Some(SourceLocation::new(tokens[1].span().source(), 3, 1))
    );
    let eof = tokens.last().expect("EOF token");
    assert_eq!(eof.span().start(), eof.span().end());
    assert_eq!(
        eof.span().start().raw(),
        " \t\r\n  let\rvalue\n".len() as u32
    );
}

#[test]
fn skips_initial_bom_and_rejects_bom_elsewhere() {
    let (sources, tokens, diagnostics) = lex_text("bom.cap", "\u{feff}let x\u{feff}");

    assert_eq!(
        non_eof_kinds(&tokens),
        vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier,
            TokenKind::Error
        ]
    );
    assert_eq!(token_texts(&sources, &tokens), vec!["let", "x", "\u{feff}"]);
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message() == "invalid character in source file"));
}

#[test]
fn recovers_after_lexical_errors_and_keeps_later_tokens() {
    let (_, tokens, diagnostics) = lex_text("recovery.cap", "$ let y = 1; \"abc\nlet z = 2;");

    assert!(diagnostics.len() >= 2);
    assert!(tokens.iter().any(|token| token.kind() == &TokenKind::Error));
    assert!(tokens.windows(4).any(|window| matches!(
        window[0].kind(),
        TokenKind::Keyword(Keyword::Let)
    ) && matches!(window[1].kind(), TokenKind::Identifier)
        && matches!(window[2].kind(), TokenKind::Operator(Operator::Equal))
        && matches!(window[3].kind(), TokenKind::Literal(LiteralKind::Integer))));
}

#[test]
fn reports_invalid_character_fixture() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/fail/invalid-character.cap");

    assert!(tokens.iter().any(|token| token.kind() == &TokenKind::Error));
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message() == "invalid character in source file"));
}

#[test]
fn reports_unterminated_string_fixture() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/fail/unterminated-string.cap");

    assert!(tokens.iter().any(|token| token.kind() == &TokenKind::Error));
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message() == "unterminated string literal"));
}

#[test]
fn reports_invalid_number_fixture() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/fail/invalid-number.cap");

    assert!(tokens.iter().any(|token| token.kind() == &TokenKind::Error));
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message() == "invalid numeric literal"));
}

#[test]
fn reports_invalid_escape_fixture() {
    let (_, tokens, diagnostics) = lex_fixture("tests/lexer/fail/invalid-escape.cap");

    assert!(tokens
        .iter()
        .any(|token| token.kind() == &TokenKind::Literal(LiteralKind::Char)));
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.message() == "invalid escape sequence"));
}

#[test]
fn invalid_inputs_produce_structured_diagnostics() {
    let mut sources = SourceMap::default();
    let source = sources.add_file(
        "invalid-inputs.cap",
        "$\n123abc\n\"unterminated\n'\\x'\n''\n'ab'\n'x\n/* missing",
    );
    let output = lex(source, sources.get(source).unwrap().text());

    assert!(output.diagnostics().len() >= 8);
    for diagnostic in output.diagnostics() {
        assert_structured_lexer_diagnostic(diagnostic);
    }
}

#[test]
fn malformed_inputs_do_not_panic() {
    for input in [
        "",
        "\u{feff}",
        "$",
        "é",
        "\"",
        "\"unterminated",
        "\"bad\\xescape\"",
        "'",
        "''",
        "'ab'",
        "'\\x'",
        "'x",
        "/*",
        "/* unterminated",
        "/**",
        "123abc",
        "1.",
        "let value = @@@;",
        "let emoji = 😀;",
        "\r\n\r\n$",
    ] {
        let result = std::panic::catch_unwind(|| {
            let mut sources = SourceMap::default();
            let source = sources.add_file("malformed.cap", input);
            let output = lex(source, sources.get(source).unwrap().text());

            assert_eq!(
                output.tokens().last().map(Token::kind),
                Some(&TokenKind::Eof)
            );
        });

        assert!(result.is_ok(), "lexer panicked for input: {input:?}");
    }
}

#[test]
fn reports_diagnostic_position_for_invalid_character() {
    let (sources, _, diagnostics) = lex_fixture("tests/lexer/fail/invalid-character.cap");

    assert_diagnostic_span(
        &sources,
        &diagnostics,
        "invalid character in source file",
        (1, 13),
        (1, 14),
        "$",
    );
}

#[test]
fn reports_diagnostic_position_for_unterminated_string() {
    let (sources, _, diagnostics) = lex_fixture("tests/lexer/fail/unterminated-string.cap");

    assert_diagnostic_span(
        &sources,
        &diagnostics,
        "unterminated string literal",
        (1, 12),
        (1, 17),
        "\"Capi",
    );
}

#[test]
fn reports_diagnostic_position_for_invalid_number() {
    let (sources, _, diagnostics) = lex_fixture("tests/lexer/fail/invalid-number.cap");

    assert_diagnostic_span(
        &sources,
        &diagnostics,
        "invalid numeric literal",
        (1, 13),
        (1, 19),
        "123abc",
    );
}

#[test]
fn reports_diagnostic_position_for_invalid_escape() {
    let (sources, _, diagnostics) = lex_fixture("tests/lexer/fail/invalid-escape.cap");

    assert_diagnostic_span(
        &sources,
        &diagnostics,
        "invalid escape sequence",
        (1, 14),
        (1, 16),
        "\\x",
    );
}

#[test]
fn reports_diagnostic_position_for_unterminated_block_comment() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("unterminated-comment.cap", "let value = 1;\n/* missing");
    let output = lex(source, sources.get(source).unwrap().text());

    assert_diagnostic_span(
        &sources,
        output.diagnostics(),
        "unterminated block comment",
        (2, 1),
        (2, 11),
        "/* missing",
    );
}

#[test]
fn reports_diagnostic_position_for_invalid_character_literals() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("invalid-chars.cap", "''\n'ab'\n'x");
    let output = lex(source, sources.get(source).unwrap().text());

    assert_diagnostic_span(
        &sources,
        output.diagnostics(),
        "empty character literal",
        (1, 1),
        (1, 3),
        "''",
    );
    assert_diagnostic_span(
        &sources,
        output.diagnostics(),
        "character literal contains more than one character",
        (2, 1),
        (2, 5),
        "'ab'",
    );
    assert_diagnostic_span(
        &sources,
        output.diagnostics(),
        "unterminated character literal",
        (3, 1),
        (3, 3),
        "'x",
    );
}

#[test]
fn reports_compile_fail_lexical_fixtures() {
    for path in [
        "tests/lexer/fail/invalid-character.cap",
        "tests/lexer/fail/invalid-number.cap",
        "tests/lexer/fail/unterminated-string.cap",
        "tests/lexer/fail/invalid-escape.cap",
        "tests/lexer/fail/unterminated-block-comment.cap",
    ] {
        let (_, tokens, diagnostics) = lex_fixture(path);

        assert!(
            !diagnostics.is_empty(),
            "{path} should produce lexical diagnostics"
        );
        assert!(
            tokens.iter().any(|token| token.kind() == &TokenKind::Error)
                || diagnostics.iter().any(|diagnostic| matches!(
                    diagnostic.message(),
                    "invalid escape sequence" | "unterminated block comment"
                )),
            "{path} should be rejected by lexical validation"
        );
    }
}

#[test]
fn rejects_non_ascii_identifier_start_for_now() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("unicode-ident.cap", "café");
    let output = lex(source, sources.get(source).unwrap().text());

    assert_eq!(sources.span_text(output.tokens()[0].span()), Some("caf"));
    assert!(output
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.message() == "invalid character in source file"));
}

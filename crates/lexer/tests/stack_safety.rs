use lexer::{tokenize, TokenKind};

#[test]
fn long_bsl_tokens_keep_kind_text_and_offset() {
    for (source, expected) in [
        ("a".repeat(73_728), TokenKind::Ident),
        ("Я".repeat(73_728), TokenKind::Ident),
        ("9".repeat(73_728), TokenKind::Decimal),
        (format!("{}.5", "9".repeat(73_728)), TokenKind::Float),
        (format!("\"{}\"", "a".repeat(73_728)), TokenKind::String),
        (format!("'{}'", "1".repeat(73_728)), TokenKind::Date),
        (format!("//{}", "a".repeat(73_728)), TokenKind::Comment),
        (" ".repeat(73_728), TokenKind::Whitespace),
        (format!("&{}", "a".repeat(73_728)), TokenKind::AnnCustom),
        (format!("#{}Если", " ".repeat(73_728)), TokenKind::PreIf),
    ] {
        let tokens = tokenize(&source);
        assert_eq!(tokens.len(), 1, "{expected:?}");
        assert_eq!(tokens[0].kind, expected);
        assert_eq!(tokens[0].text.as_str(), source);
        assert_eq!(tokens[0].offset, 0);
    }
}

#[test]
fn long_sdbl_tokens_keep_source_without_stack_growth() {
    for source in [
        "a".repeat(73_728),
        "9".repeat(73_728),
        " ".repeat(73_728),
        format!("//{}", "a".repeat(73_728)),
        format!("\"{}\"", "a".repeat(73_728)),
    ] {
        let tokens = lexer::sdbl::tokenize_sdbl(&source);
        let mut offset = 0;
        for token in &tokens {
            assert_eq!(token.offset, offset);
            offset += token.text.len();
        }
        assert_eq!(offset, source.len());
        assert_eq!(tokens.iter().map(|t| t.text.as_str()).collect::<String>(), source);
    }
}

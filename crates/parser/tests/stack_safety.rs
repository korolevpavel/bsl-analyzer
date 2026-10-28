//! Long lexemes and nested grammar must produce a tree or a located diagnostic,
//! never consume the process stack. Uses the default test-thread stack.
#[test]
fn long_tokens_preserve_all_source_and_produce_diagnostics() {
    for source in [
        "a".repeat(73_728),
        "Я".repeat(73_728),
        format!("//{}\nПроцедура А()", "x".repeat(73_728)),
        format!("Сообщить(\"{}\");", "x".repeat(73_728)),
        format!("{}Процедура А()", " ".repeat(73_728)),
    ] {
        let parsed = parser::parse(&source);
        assert_eq!(parsed.syntax_node().text().to_string(), source);
        assert!(parsed.syntax_node().descendants().count() > 0);
        if !source.starts_with("Сообщить") {
            assert!(!parsed.errors().is_empty());
        }
    }
}

#[test]
fn iterative_unary_operators_complete_without_stack_growth() {
    for prefix in ["Не ", "+ ", "- "] {
        let source = format!("А = {}Истина;", prefix.repeat(10_000));
        let parsed = parser::parse(&source);
        if prefix == "Не " {
            assert!(parsed.errors().iter().any(|e| e.message().contains("tree depth limit")));
        } else {
            assert!(parsed.errors().is_empty(), "{:?}", parsed.errors());
        }
        assert_eq!(parsed.syntax_node().text().to_string(), source);
        assert!(parsed.syntax_node().descendants().count() > 0);
    }
}

#[test]
fn actual_recursive_nesting_exhaustion_has_a_location_and_keeps_source() {
    let cases = [
        format!("А = {}1{};", "(".repeat(2_000), ")".repeat(2_000)),
        format!("{}{}", "Если Истина Тогда\n".repeat(2_000), "КонецЕсли;\n".repeat(2_000)),
        format!("#Если {}Сервер Тогда\n#КонецЕсли\n", "Не ".repeat(2_000)),
        format!("{}{}", "#Если Сервер Тогда\n".repeat(2_000), "#КонецЕсли\n".repeat(2_000)),
    ];
    for source in cases {
        let parsed = parser::parse(&source);
        assert_eq!(parsed.syntax_node().text().to_string(), source);
        assert!(parsed.syntax_node().descendants().count() > 0);
        let error = parsed
            .errors()
            .iter()
            .find(|e| e.message().contains("nesting limit reached"))
            .unwrap_or_else(|| {
                panic!(
                    "exhaustion diagnostic for {:?}: {:?}",
                    &source[..source.len().min(40)],
                    parsed.errors()
                )
            });
        assert!(u32::from(error.range().start()) > 0);
    }
}

#[test]
fn mixed_nesting_uses_one_depth_budget_and_shallow_nesting_stays_valid() {
    for count in [8, 40] {
        let source = format!(
            "{}{}А = {}1{};\n{}{}",
            "#Если Сервер Тогда\n".repeat(count),
            "Если Истина Тогда\n".repeat(count),
            "(".repeat(count),
            ")".repeat(count),
            "КонецЕсли;\n".repeat(count),
            "#КонецЕсли\n".repeat(count)
        );
        let parsed = parser::parse(&source);
        assert_eq!(parsed.syntax_node().text().to_string(), source);
        if count == 8 {
            assert!(parsed.errors().is_empty(), "{:?}", parsed.errors());
        } else {
            assert!(parsed.errors().iter().any(|e| e.message().contains("nesting limit reached")));
        }
    }
}

#[test]
fn long_iterative_binary_and_postfix_trees_support_traversal_and_drop() {
    for source in
        [format!("А = 1{};", "+1".repeat(36_864)), format!("А = Б{};", ".В".repeat(24_576))]
    {
        let parsed = parser::parse(&source);
        assert!(parsed.errors().iter().any(|e| e.message().contains("tree depth limit")));
        assert_eq!(parsed.syntax_node().text().to_string(), source);
        assert_eq!(parsed.syntax_node().descendants().count(), 2);
    }
}

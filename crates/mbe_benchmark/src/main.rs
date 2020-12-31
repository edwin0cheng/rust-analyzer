use mbe::{ast_to_token_tree, parse_to_token_tree, ExpandError, MacroRules};
use syntax::{ast, AstNode};

fn main() {
    bench_expansion_only();
}

fn bench_expansion_only() {
    let ra_fixture = r#"
    macro_rules! m {
        ($id:ident) => { let $id = 0; }
    }
"#;
    let invocation = r#"m!(a)"#;

    let definition_tt = parse(ra_fixture);

    let source_file = ast::SourceFile::parse(invocation).tree();
    let macro_invocation =
        source_file.syntax().descendants().find_map(ast::MacroCall::cast).unwrap();
    let (invocation_tt, _) = ast_to_token_tree(&macro_invocation.token_tree().unwrap())
        .ok_or_else(|| ExpandError::ConversionError)
        .unwrap();

    let time = std::time::Instant::now();    
    for _ in 0..1000000 {
        let rules = MacroRules::parse(&definition_tt).unwrap();
        rules.expand(&invocation_tt).result().unwrap();
    }
    eprintln!("time used: {}ms", time.elapsed().as_millis());

    fn parse(ra_fixture: &str) -> tt::Subtree {
        let source_file = ast::SourceFile::parse(ra_fixture).ok().unwrap();
        let macro_definition =
            source_file.syntax().descendants().find_map(ast::MacroRules::cast).unwrap();

        let (definition_tt, _) =
            ast_to_token_tree(&macro_definition.token_tree().unwrap()).unwrap();

        let parsed = parse_to_token_tree(
            &ra_fixture[macro_definition.token_tree().unwrap().syntax().text_range()],
        )
        .unwrap()
        .0;
        assert_eq!(definition_tt, parsed);

        definition_tt
    }
}

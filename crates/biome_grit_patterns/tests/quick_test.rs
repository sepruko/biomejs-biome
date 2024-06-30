use biome_grit_parser::parse_grit;
use biome_grit_patterns::{GritQuery, GritTargetFile, GritTargetLanguage, JsTargetLanguage};
use biome_js_parser::{parse, JsParseOptions};
use biome_js_syntax::JsFileSource;

// Use this test to quickly execute a Grit query against a source snippet.
#[ignore]
#[test]
fn test_query() {
    let parse_grit_result = parse_grit("`hello`");
    if !parse_grit_result.diagnostics().is_empty() {
        panic!("Cannot parse query:\n{:?}", parse_grit_result.diagnostics());
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
    )
    .expect("could not construct query");

    if !query.diagnostics.is_empty() {
        println!("Diagnostics from compiling query:\n{:?}", query.diagnostics);
    }

    let body = r#"
function hello() {
    console
        .log("hello");
}
"#;

    let file = GritTargetFile {
        path: "test.js".into(),
        parse: parse(body, JsFileSource::tsx(), JsParseOptions::default()).into(),
    };
    let results = query.execute(file).expect("could not execute query");

    println!("Results: {results:?}");
}

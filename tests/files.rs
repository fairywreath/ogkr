use ogkr_rs::{lex::tokenize, parse::parse_tokens};

fn test_file(source: &str) {
    let tokens = tokenize(source).expect("must be tokenized");
    let _raw_ogkr = parse_tokens(tokens).expect("must be parsed");
}

#[test]
fn test_1() {
    test_file(include_str!("../charts/1.ogkr"));
}

#[test]
fn test_2() {
    test_file(include_str!("../charts/2.ogkr"));
}

#[test]
fn test_3() {
    test_file(include_str!("../charts/3.ogkr"));
}

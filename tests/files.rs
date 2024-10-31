use ogkr_rs::lex::tokenize;

#[test]
fn test_1() {
    let source = include_str!("../charts/1.ogkr");
    tokenize(source).expect("must be parsed");
}

#[test]
fn test_2() {
    let source = include_str!("../charts/2.ogkr");
    tokenize(source).expect("must be parsed");
}

#[test]
fn test_3() {
    let source = include_str!("../charts/3.ogkr");
    tokenize(source).expect("must be parsed");
}


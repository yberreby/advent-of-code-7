use FastHashMap;
use run_source;

#[test]
fn test_run_full_program() {
    let program_source = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 \
                          -> g
NOT x -> h
NOT y -> i";

    let output = run_source(program_source);

    let mut expected = FastHashMap::default();
    expected.insert("d".into(), 72);
    expected.insert("e".into(), 507);
    expected.insert("f".into(), 492);
    expected.insert("g".into(), 114);
    expected.insert("h".into(), 65412);
    expected.insert("i".into(), 65079);
    expected.insert("x".into(), 123);
    expected.insert("y".into(), 456);

    assert_eq!(output, expected);
}


#[test]
fn test_run_challenge() {
    let source: &'static str = include_str!("../source.txt");
    let expected = 46065;

    assert_eq!(*run_source(source).get("a").unwrap(), expected);
}

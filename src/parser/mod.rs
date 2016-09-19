pub mod sql;
pub mod ast;

#[test]
fn simple_parse() {
    let thing = sql::parse_Statement("SELECT * FROM foo;");
    println!("{:?}", thing);
    assert!(sql::parse_Statement("SELECT 1;").is_ok());
    assert!(thing.is_ok());
    assert!(sql::parse_Statement("SELECT * FROM foo WHERE 1=1;").is_ok());
}

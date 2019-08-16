extern crate rusty_ci;
use rusty_ci::{unquote};


#[test]
fn unquote_string() {
    assert_eq!(
        unquote("\"\"hey jude\""),
        String::from("\"hey jude")
    );

    assert_eq!(
        unquote("\"\"hey jude\"\""),
        String::from("\"hey jude\"")
    );

    assert_eq!(
        unquote("'hey jude'"),
        String::from("hey jude")
    );
    
    assert_eq!(
        unquote("''hey jude'"),
        String::from("'hey jude")
    );

    assert_eq!(
        unquote("'hey jude"),
        String::from("'hey jude")
    );
    assert_eq!(
        unquote("\"hey jude"),
        String::from("\"hey jude")
    );
    assert_eq!(
        unquote("hey jude'"),
        String::from("hey jude'")
    );
    assert_eq!(
        unquote("hey jude\""),
        String::from("hey jude\"")
    );
    assert_eq!(
        unquote("''hey jude''"),
        String::from("'hey jude'")
    );

}

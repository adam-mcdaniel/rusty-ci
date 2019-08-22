extern crate rusty_yaml;
use rusty_yaml::Yaml;

extern crate rusty_ci;
use rusty_ci::{unmatched_quotes, unwrap};

#[test]
fn unmatched_quotes_yaml() {
    let yaml = Yaml::from(
        r#"

testing:
  - "Test"
  - 2.0
  - "this line will fail
  - okay

"#,
    );

    assert_eq!(
        unmatched_quotes(&yaml),
        Some(String::from(r#"  - "this line will fail"#))
    );

    let yaml = Yaml::from(
        r#"

testing:
  - "Test"
  - 2.0
  - okay

"#,
    );

    assert_eq!(unmatched_quotes(&yaml), None);
}

#[test]
fn unwrap_yaml() {
    let yaml = Yaml::from(
        r#"
testing: "hey there"
key: 1.0
value: https://github.com/adam-mcdaniel/rusty-ci
"#,
    );

    assert_eq!(unwrap(&yaml, "testing"), String::from("hey there"));
    assert_eq!(unwrap(&yaml, "key"), String::from("1.0"));
    assert_eq!(
        unwrap(&yaml, "value"),
        String::from("https://github.com/adam-mcdaniel/rusty-ci")
    );
}

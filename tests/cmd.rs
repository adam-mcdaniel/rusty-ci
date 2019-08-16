extern crate rusty_ci;
use rusty_ci::Cmd;

#[test]
fn cmd_test() {
    let cmd = Cmd::new("rusty-ci").arg("build").arg("--help");

    assert_eq!(
        cmd,
        Cmd {
            program: String::from("rusty-ci"),
            args: vec![String::from("build"), String::from("--help")]
        }
    )
}

//
// Unit tests for build Step operations
//

use rusty_ci::step::Step;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_to_string() {
        assert_eq!(
            Step::command("make -j 8", None).to_string(),
            format!("steps.ShellCommand(command={:?})", vec!["make", "-j", "8"])
        );


        assert_eq!(
            Step::command("make -j 8", Some("build")).to_string(),
            format!(
                "steps.ShellCommand(command={:?}, workdir=\"build\")",
                vec!["make", "-j", "8"]
            )
        );
    }
}
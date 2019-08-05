#[macro_use]
extern crate rusty_ci;
use rusty_ci::Step;

#[test]
fn command_step() {
    assert_eq!(
        Step::command(
            "cmake ..",
            Some("build")
        ).to_string(),
        format!(
            "steps.ShellCommand(command={:?}, workdir=\"{}\")",
            vec!["cmake", ".."],
            "build"
        )
    );
}


#[test]
fn git_clone_step() {
    let url = "https://github.com/adam-mcdaniel/rusty-ci";
    assert_eq!(
        Step::git_clone(url).to_string(),
        format!(
            "steps.Git(repourl=\"{}\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True)",
            url
        )
    );
}


#[test]
fn git_lab_clone_step() {
    let url = "https://gitlab.com/adam-mcdaniel/rusty-ci";
    assert_eq!(
        Step::gitlab_clone(url).to_string(),
        format!(
            "steps.GitLab(repourl=\"{}\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True)",
            url
        )
    );
}
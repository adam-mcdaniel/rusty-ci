extern crate rusty_yaml;
use rusty_yaml::Yaml;

extern crate rusty_ci;
use rusty_ci::Builder;


#[test]
fn builder_from_yaml() {
    let yaml = Yaml::from(r#"xasm-build:
  workers:
    - xasm-worker
    
  script:
    - python main.py
    - echo XASM build done!

  repo: "https://github.com/adam-mcdaniel/xasm"
"#).get_section("xasm-build").unwrap();

    let builder = Builder::from(yaml);
    let output = builder.to_string();

    assert_eq!(
        output,
        "
temp_factory = util.BuildFactory()
temp_factory.addStep(steps.Git(repourl=\"https://github.com/adam-mcdaniel/xasm\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True))
temp_factory.addStep(steps.GitLab(repourl=\"https://github.com/adam-mcdaniel/xasm\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True))
temp_factory.addStep(steps.ShellCommand(command=[\"python\", \"main.py\"], workdir=\"./build\"))
temp_factory.addStep(steps.ShellCommand(command=[\"echo\", \"XASM\", \"build\", \"done!\"], workdir=\"./build\"))
c['builders'].append(
    util.BuilderConfig(name=\"xasm-build\",
    workernames=[\"xasm-worker\"],
    factory=temp_factory))
        ".to_string()
    )
}
extern crate rusty_yaml;
use rusty_yaml::Yaml;

extern crate rusty_ci;
use rusty_ci::Scheduler;


#[test]
fn scheduler_from_yaml() {
    let yaml = Yaml::from(r#"xasm-change:
  builders:
    - xasm-build
    - xasm-test

  branch: ".*"
  triggers:
    - '.*\.py'
    - '.*\.cpp'
    - '.*Makefile'
    - '.*CMakeLists.txt'
  password: "ok to test"
"#).get_section("xasm-change").unwrap();

    let scheduler = Scheduler::from(yaml);
    let output = scheduler.to_string();

    assert_eq!(
        output,
        "\n@util.renderer\ndef xasm_change_triggers(props):\n    builders = [\"xasm-build\", \"xasm-test\"]\n\n    triggers = [\".*\\\\.py\", \".*\\\\.cpp\", \".*Makefile\", \".*CMakeLists.txt\"]\n\n    if not is_whitelisted(props, \"ok to test\"):\n        print(\'NOT WHITELISTED!!!\')\n        return []\n\n    for f in props.files:\n        for regex in triggers:\n            print(\"FILE:   \", f)\n            print(\"TRIGGER:\", regex)\n            if re.fullmatch(regex, str(f)):\n                return builders\n\n    return []\n\n\nxasm_change = schedulers.AnyBranchScheduler(name=\"xasm_change\",\n    change_filter=util.ChangeFilter(branch_re=\".*\"),\n    builderNames=xasm_change_triggers)\n\nc[\'schedulers\'].append(xasm_change)\n\nc[\'schedulers\'].append(schedulers.ForceScheduler(name=\"force_xasm_change\",\n    builderNames=[\"xasm-build\", \"xasm-test\"]))\n"
    );
}
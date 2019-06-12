use rusty_yaml::Yaml;
use std::process::exit;
use std::fmt::{Display, Error, Formatter};


#[allow(dead_code)]
pub struct Scheduler {
    // Name of scheduler
    name: String,

    // A regex expr that accepts a branch name.
    // This scheduler will only operate on the
    // branches with names that match this regex.
    branch: String,

    // Regex exprs accepting file names.
    // When these file names are changed in a branch,
    // They will trigger the scheduler's workers.
    file_triggers: Vec<String>,

    // The builders to trigger
    buildernames: Vec<String>,
}


impl Scheduler {
    // Create new scheduler
    fn new<S>(name: S, branch: S, file_triggers: Vec<S>, buildernames: Vec<S>) -> Self
    where
        S: Display,
    {
        Self {
            name: name.to_string(),
            branch: branch.to_string(),
            file_triggers: file_triggers.iter().map(|s| s.to_string().trim().trim_start_matches("\"").trim_end_matches("\"").to_string()).collect(),
            buildernames: buildernames.iter().map(|s| s.to_string().trim().trim_start_matches("\"").trim_end_matches("\"").to_string()).collect(),
        }
    }
}


impl Display for Scheduler {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "
@util.renderer
def {name}_triggers(props):
    builders = {buildernames}

    triggers = {triggers}

    for f in props.files:
        for regex in triggers:
            if re.match(regex, str(f)):
                return builders

    return []

c['schedulers'].append(schedulers.AnyBranchScheduler(name=\"{name}\",
    change_filter=util.ChangeFilter(branch_re=\"{branch}\"),
    builderNames={name}_triggers))

c['schedulers'].append(schedulers.ForceScheduler(name=\"force_{name}\",
    builderNames={buildernames}))
",
            name = self.name.replace("-", "_"),
            branch = self.branch.trim_start_matches("\"").trim_end_matches("\""),
            triggers = format!("{:?}", self.file_triggers)
                .replace("\\\"", "")
                .replace("\\\\\\\\", "\\\\"),
            buildernames = format!("{:?}", self.buildernames)
        )
    }
}


impl From<Yaml> for Scheduler {
    fn from(yaml: Yaml) -> Self {
        let name = yaml.get_name();

        for section in ["branch", "triggers", "builders"].iter() {
            if !yaml.has_section(section) {
                println!("There was an error creating a scheduler: The '{}' section is not specified for '{}'", section, name);
                exit(1);
            }
        }


        let branch: String = yaml.get_section("branch").unwrap().nth(0).unwrap().to_string();

        let mut triggers = vec![];
        for trigger in yaml.get_section("triggers").unwrap() {
            triggers.push(trigger.to_string());
        }


        let mut builders = vec![];
        for builder in yaml.get_section("builders").unwrap() {
            builders.push(builder.to_string());
        }

        Scheduler::new(name, branch, triggers, builders)
    }
}

/// Because we cannot impl<T, E> From <T> for Result<T, E>,
/// we cannot use result for from trait definitions.
/// 
/// This means we must handle the error some other way.
/// For now, Im using exit. I might 
/// change this in the future, as needed.
use std::process::exit;
use std::fmt::{Display, Error, Formatter};

use crate::{Step, unwrap};
use rusty_yaml::Yaml;
use std::path::PathBuf;

/// This represents the directory in which buildbot starts the
/// instance of the builder running your script.
/// All this constant is used for is prepending
/// to the working dir for all paths.
const START_DIR: &str = "./build";


/// The Builder struct encapsulates all the operations involved in
/// defining a builder in buildbot. A builder works by giving tasks
/// called steps to workers.
/// 
/// A Builder object is composed of the name of the builder, the list
/// of worker names that the builder will give the steps to, and the 
/// steps themselves.
pub struct Builder {
    name: String,
    workernames: Vec<String>,
    steps: Vec<Step>,
}


/// The implmentation of the Builder struct
impl Builder {
    /// Create a new builder from a name, a list of worker names, and a list of steps
    fn new<S: Display>(name: S, workernames: Vec<S>, steps: Vec<Step>) -> Self {
        Self {
            name: name.to_string(),
            workernames: workernames.iter().map(|s| s.to_string()).collect(),
            steps: steps,
        }
    }

    /// This method returns the name of the builder
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}


/// This impl converts a Builder into the Python code for buildbot that will
/// give us the behaviour we want.
impl Display for Builder {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "
temp_factory = util.BuildFactory()
{steps}
c['builders'].append(
    util.BuilderConfig(name=\"{name}\",
    workernames={:?},
    factory=temp_factory))
        ",
            self.workernames,
            name = self.name,
            steps = self
                .steps
                .iter()
                .map(|s| { format!("temp_factory.addStep({})", s) })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

/// This impl takes a rust-yaml::Yaml object and converts it into a Builder object
impl From<Yaml> for Builder {
    fn from(yaml: Yaml) -> Builder {
        // The name of the yaml section will be used as the name of the builder
        let name = yaml.get_name();

        // Verify that the yaml contains the `workers`, `script`, and `repo` sections
        // If not, tell user, and exit with error code 1.
        for section in ["workers", "script", "repo"].iter() {
            if !yaml.has_section(section) {
                error!("There was an error creating a builder: '{}' section not specified for '{}'", section, name);
                exit(1);
            }
        }
        // Now that we've verified the required sections exist, continue

        let mut steps: Vec<Step> = vec![];

        // Because of the way buildbot processes shell commands,
        // you cannot call the change directory, or cd command as an instruction.
        // Well, you can, but it wont change the directory.
        //  
        // To fix this, we keep track of the current working directory using a PathBuf.
        // When the script uses the `cd` command, it will modify this path.
        let mut workdir = PathBuf::new();
        // We want to start in the builders starting directory
        workdir.push(START_DIR);



        // Get the url for the repo from the yaml section
        let url= unwrap(&yaml, "repo");

        // Refresh your copy of the repository
        steps.push(Step::git_clone(url));

        // Run each instruction in the script section
        for instruction in yaml.get_section("script").unwrap() {
            // Here we turn the instruction into a slice of each word so we can match it
            match instruction
                .to_string()
                .split_whitespace()
                .collect::<Vec<&str>>()[..]
            {
                ["cd", path] => workdir.push(path),
                _ => steps.push(Step::command(
                    instruction.to_string(),
                    match workdir.to_str() {
                        Some(s) => Some(s.to_string()),
                        None => None,
                    },
                )),
            };
        }

        // Get the workers from the yaml file
        let mut workers: Vec<String> = vec![];
        for worker in yaml.get_section("workers").unwrap() {
            workers.push(worker.to_string());
        }

        // Return the new builder
        Builder::new(name, workers, steps)
    }
}

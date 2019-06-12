use std::process::exit;
use std::fmt::{Display, Error, Formatter};

use crate::step::Step;
use rusty_yaml::Yaml;
use std::path::PathBuf;


const START_DIR: &str = "./build";


#[allow(dead_code)]
pub struct Builder {
    name: String,
    workernames: Vec<String>,
    steps: Vec<Step>,
}


// Implementation for Builder struct
impl Builder {
    // Create new builder
    fn new<S>(name: S, workernames: Vec<S>, steps: Vec<Step>) -> Self
    where
        S: Display,
    {
        Self {
            name: name.to_string(),
            workernames: workernames.iter().map(|s| s.to_string()).collect(),
            steps: steps,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}


// How to convert Builder to a String / how to Format
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


impl From<Yaml> for Builder {
    fn from(yaml: Yaml) -> Builder {
        let name = yaml.get_name();

        for section in ["workers", "script", "repo"].iter() {
            if !yaml.has_section(section) {
                println!("There was an error creating a scheduler: {} section not specified for {} builder", section, name);
                exit(1);
            }
        }


        let mut steps: Vec<Step> = vec![];
        let mut workdir = PathBuf::new();
        workdir.push(START_DIR);
        let mut workers: Vec<String> = vec![];

        let url: String =
            yaml.get_section("repo").unwrap().nth(0).unwrap().to_string();

        steps.push(Step::git_clone(url));


        for instruction in yaml.get_section("script").unwrap() {
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

        for worker in yaml.get_section("workers").unwrap() {
            workers.push(worker.to_string());
        }


        Builder::new(name, workers, steps)
    }
}

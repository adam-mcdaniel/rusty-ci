use crate::{MasterConfig, Worker};

use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};


pub struct Makefile {
    master: MasterConfig,
    workers: Vec<Worker>,
}


impl Makefile {
    pub fn new(master: MasterConfig, workers: Vec<Worker>) -> Self {
        Self { master, workers }
    }
}


impl From<Yaml> for Makefile {
    fn from(yaml: Yaml) -> Self {
        let mut workers = vec![];
        for worker in yaml.get_section("workers") {
            workers.push(Worker::from(worker));
        }

        let master = MasterConfig::from(yaml);
        Self::new(master, workers)
    }
}

impl Display for Makefile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "
workerdirs := {worker_dirs}
masterdir := \"master\"


all: install start

start:
\tbuildbot stop master
\tbuildbot reconfig master
\tbuildbot cleanupdb master
\t. venv/bin/activate; buildbot start master;
\t-. venv/bin/activate; $(foreach dir,$(workerdirs),buildbot-worker restart $(dir);)


install:
\tsudo apt-get install python3-dev
\tsudo apt-get install python3-pip

\tpython3 -m venv venv
\t. venv/bin/activate; python3 -m pip install -U pip; python3 -m pip install 'buildbot[bundle]';
\t. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial
\t. venv/bin/activate; buildbot create-master $(masterdir);
\techo \"{master_setup}\" > $(masterdir)/master.cfg
\t{workers_setup}
",
            master_setup = self
                .master
                .to_string()
                .replace("\n", "\\n")
                .replace("\"", "'"),
            worker_dirs = self
                .workers
                .iter()
                .map(|w| w.get_dir())
                .collect::<Vec<String>>()
                .join(" "),
            workers_setup = self
                .workers
                .iter()
                .map(|w| format!(
                    "
\tmkdir -p {worker_dir}
\t-. venv/bin/activate; buildbot-worker create-worker {worker_dir} localhost {worker_dir} pass
\techo \"{script}\" > {worker_dir}/buildbot.tac",
                    worker_dir = w.get_dir(),
                    script = w.to_string().replace("\n", "\\n").replace("\"", "'")
                ))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}
use crate::unwrap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;


/// This struct holds the information that is used to build the worker `buildbot.tac` file
/// Each worker has:
/// - a name that is used by the builders to assign work,
/// - a password that the master uses to access the worker
/// - a working directory name that the bot will be created in
/// - the host address of the master bot, the ip
/// - the port of the master bot
#[derive(Clone)]
pub struct Worker {
    name: String,
    dir: String,
    password: String,
    masterhost: String,
    masterport: String,
}


impl Worker {
    fn new<S: ToString>(name: S, dir: S, password: S, masterhost: S, masterport: S) -> Self {
        Self {
            name: name.to_string(),
            dir: dir.to_string(),
            password: password.to_string(),
            masterhost: masterhost.to_string(),
            masterport: masterport.to_string(),
        }
    }

    /// Retrieves the name field of the struct
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Retrieves the working dir field of the struct
    pub fn get_dir(&self) -> String {
        self.dir.clone()
    }

    /// Retrieves the password field of the struct
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}


/// Convert a Yaml section to a Worker
///
/// The worker requires that the yaml section has the following subsections:
/// `masterhost`, `masterport`, `password`, and `basedir`.
/// Masterhost holds the host address of the master bot,
/// Masterport hold the host port of the master bot.
/// Basedir holds the path of the working directory of the bot
impl From<Yaml> for Worker {
    fn from(yaml: Yaml) -> Self {
        let name = yaml.get_name();

        for section in ["master-ip", "dir"].iter() {
            if !yaml.has_section(section) {
                error!("There was an error creating a worker: The '{}' section is not specified for '{}'", section, name);
                exit(1);
            }
        }

        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        let basedir = unwrap(&yaml, "dir");
        let masterhost = unwrap(&yaml, "master-ip");

        // Now, instead of getting the master port from the Yaml object, we just use 9989.
        Self::new(name, basedir, password, masterhost, String::from("9989"))
    }
}

/// This is similar to the Display impl for the MasterConfig struct.
/// This returns the Python `buildbot.tac` file for an individual worker.
impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            r#"import os

from buildbot_worker.bot import Worker
from twisted.application import service

basedir = '{basedir}'
rotateLength = 10000000
maxRotatedFiles = 10

# if this is a relocatable tac file, get the directory containing the TAC
if basedir == '.':
    import os.path
    basedir = os.path.abspath(os.path.dirname(__file__))

# note: this line is matched against to check that this is a worker
# directory; do not edit it.
application = service.Application('buildbot-worker')

from twisted.python.logfile import LogFile
from twisted.python.log import ILogObserver, FileLogObserver
logfile = LogFile.fromFullPath(
    os.path.join(basedir, "twistd.log"), rotateLength=rotateLength,
    maxRotatedFiles=maxRotatedFiles)
application.setComponent(ILogObserver, FileLogObserver(logfile).emit)

buildmaster_host = '{masterhost}'
port = {masterport}
workername = '{name}'
passwd = '{password}'
keepalive = 600
umask = None
maxdelay = 300
numcpus = None
allow_shutdown = None
maxretries = None

s = Worker(buildmaster_host, port, workername, passwd, basedir,
           keepalive, umask=umask, maxdelay=maxdelay,
           numcpus=numcpus, allow_shutdown=allow_shutdown,
           maxRetries=maxretries)
s.setServiceParent(application)

"#,
            name = self.name,
            basedir = self.dir,
            password = self.password,
            masterhost = self.masterhost,
            masterport = self.masterport
        )
    }
}

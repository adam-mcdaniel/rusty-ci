use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};


pub struct Worker {
    name: String,
    dir: String,
    password: String,
    masterhost: String,
    masterport: String,
}


impl Worker {
    fn new<S: ToString>(name: S, dir: S, password: S, masterhost: S) -> Self {
        Self {
            name: name.to_string(),
            dir: dir.to_string(),
            password: password.to_string(),
            masterhost: masterhost.to_string(),
            masterport: String::from("9989"),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_dir(&self) -> String {
        self.dir.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}


impl From<Yaml> for Worker {
    fn from(yaml: Yaml) -> Self {
        let name = yaml.get_name();

        for section in ["masterhost", "basedir", "password"].iter() {
            assert!(
                yaml.has_section(section),
                format!("{} section not specified for {} worker", section, name)
            )
        }

        let basedir = yaml
            .get_section("basedir")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();
        let password = yaml
            .get_section("password")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();
        let masterhost = yaml
            .get_section("masterhost")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

        Self::new(name, basedir, password, masterhost)
    }
}


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
s.setServiceParent(application)"#,
            name = self.name,
            basedir = self.dir,
            password = self.password,
            masterhost = self.masterhost,
            masterport = self.masterport
        )
    }
}

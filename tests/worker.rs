extern crate rusty_yaml;
use rusty_yaml::Yaml;

extern crate rusty_ci;
use rusty_ci::Worker;


#[test]
fn worker_from_yaml() {
    let yaml = Yaml::from(r#"xasm-worker:
  masterhost: localhost
  masterport: 9989
  basedir: '/home/adam/Desktop/rusty-ci/testing/xasm-worker'
  password: pass"#
  ).get_section("xasm-worker").unwrap();


  let worker = Worker::from(yaml);
  assert_eq!("import os

from buildbot_worker.bot import Worker
from twisted.application import service

basedir = '/home/adam/Desktop/rusty-ci/testing/xasm-worker'
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
    os.path.join(basedir, \"twistd.log\"), rotateLength=rotateLength,
    maxRotatedFiles=maxRotatedFiles)
application.setComponent(ILogObserver, FileLogObserver(logfile).emit)

buildmaster_host = 'localhost'
port = 9989
workername = 'xasm-worker'
passwd = 'pass'
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

".to_string(), worker.to_string()
)
//   println!("'{}'", worker)
}
use std::fs::File;
use std::io::prelude::*;
use crate::buildsystem::BuildSystem;

pub struct BashBuildSystem;
impl BashBuildSystem {
    pub fn new() -> Self { Self {} }
}

impl BuildSystem for BashBuildSystem {
    fn install(&self) -> Result<(), String> {

        let mut bash = File::create("install.sh").unwrap();
        
        writeln!(bash,"
sudo apt-get install python3-dev -y
sudo apt-get install python3-pip -y
sudo apt-get install python3-venv -y

python3 -m venv venv
. venv/bin/activate; python3 -m pip install -U pip; python3 -m pip install 'buildbot[bundle]';
. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial
"
        );

        Ok(())
    }

    fn install_python(&self) -> Result<(), String> {Ok(())}
    fn install_buildbot(&self) -> Result<(), String> {Ok(())}
}
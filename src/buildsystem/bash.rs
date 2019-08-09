use crate::buildsystem::BuildSystem;
use crate::{yes_or_no, File, AUTH_TOKEN_PATH};
use std::process::exit;

/// This is the best build system rusty-ci supports right now.
/// It writes the dependency installation instructions to a shell script file,
/// And tells you how to use them.
/// The process for building the master and the workers is set to the default.
pub struct Bash;
impl Bash {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildSystem for Bash {
    /// Writes install script to `install.sh` for user to run
    fn install(&mut self) -> Result<(), String> {
        if !yes_or_no(
            "Do you already have python3-dev, python3-pip, and python3-venv installed? (y/n) ",
        ) {
            error!("You must install those packages before continuing!");
            exit(0);
        }
        info!("Writing install file to `./install.sh`");
        File::write(
            "install.sh",
            "#!/bin/sh

python3 -m venv venv 2>&1
. venv/bin/activate

python3 -m pip install -U pip >/dev/null
python3 -m pip install txrequests treq 'buildbot[bundle]' >/dev/null
python3 -m pip install buildbot-worker setuptools-trial >/dev/null
",
        )?;
        info!("Successfully wrote install file");
        warn!("To install dependencies run `install.sh`");
        warn!("Before building from a YAML file, be sure to run `. venv/bin/activate`");
        info!(
            "Next, write your VCS's api token to '{}', and then run the `build` subcommand",
            AUTH_TOKEN_PATH
        );
        Ok(())
    }

    fn install_python(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn install_buildbot(&mut self) -> Result<(), String> {
        Ok(())
    }
}

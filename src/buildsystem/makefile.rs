use crate::{yes_or_no, File};
use crate::{BuildSystem, AUTH_TOKEN_PATH};
use std::process::exit;
pub struct Makefile;

impl Makefile {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildSystem for Makefile {
    /// Write the makefile
    fn install(&mut self) -> Result<(), String> {
        if !yes_or_no(
            "Do you already have python3-dev, python3-pip, and python3-venv installed? (y/n) ",
        ) {
            error!("You must install those packages before continuing!");
            exit(0);
        }
        info!("Writing install file to `./Makefile`");
        File::write("Makefile", ".SILENT: install

install:
\tpython3 -m venv venv
\t. venv/bin/activate; python3 -m pip install -U pip >/dev/null; python3 -m pip install txrequests treq 'buildbot[bundle]' >/dev/null;
\t. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial >/dev/null
")?;
        info!("Successfully wrote Makefile");
        warn!("To install dependencies run `make install`");
        info!(
            "Next, write your VCS's api token to '{}', and then run the `build` subcommand",
            AUTH_TOKEN_PATH
        );
        Ok(())
    }

    /// Prompts user to confirm they've already ran the install subcommand    
    fn prebuild(&mut self) -> Result<(), String> {
        if yes_or_no("Did you already run the install subcommand? (y/n) ") {
            Ok(())
        } else {
            error!("You must run the install subcommand before the build subcommand!");
            exit(0);
        }
    }

    fn install_python(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn install_buildbot(&mut self) -> Result<(), String> {
        Ok(())
    }
}

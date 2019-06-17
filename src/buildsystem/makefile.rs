
use crate::BuildSystem;
use crate::{yes_or_no, File};
use std::process::exit;
pub struct Makefile;

impl Makefile {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildSystem for Makefile {
    fn install(&mut self) -> Result<(), String> {
        info!("Writing install file to `./Makefile`");
        File::write("Makefile", "
install:
\tsudo apt-get install python3-dev -y
\tsudo apt-get install python3-pip -y
\tsudo apt-get install python3-venv -y

\tpython3 -m venv venv
\t. venv/bin/activate; python3 -m pip install -U pip; python3 -m pip install txrequests treq 'buildbot[bundle]';
\t. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial
")?;
        info!("Successfully wrote Makefile");
        warn!("To install dependencies run `make install`");
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
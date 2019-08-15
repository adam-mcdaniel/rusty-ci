use crate::buildsystem::BuildSystem;
use crate::{Cmd, File, MasterConfig, Worker, AUTH_TOKEN_PATH};

/// This struct is identical to the Bash buildsystem,
/// except that it does not confirm anything with the user at all.
/// This is meant to be used by scripts that automate the usage of rusty-ci.
#[derive(Default)]
pub struct Quiet;

impl BuildSystem for Quiet {
    /// Rebuild master without killing any running processes
    fn rebuild(&mut self, master: MasterConfig) -> Result<(), String> {
        self.prebuild()?; // Call the prebuild method

        info!("Creating master...");
        self.create_master()?;

        let workers = master.get_workers();
        info!("Creating workers...");
        self.create_workers(&workers)?;
        info!("Writing to master/master.cfg...");
        self.write_master_config(&master)?;
        info!("Writing to worker configs...");
        self.write_worker_configs(&workers)?;
        info!("Reconfiguring master...");
        self.reconfigure_master()?;
        Ok(())
    }

    /// Writes install script to `install.sh` for user to run
    fn install(&mut self) -> Result<(), String> {
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

    fn build(&mut self, master: MasterConfig) -> Result<(), String> {
        self.prebuild()?; // Call the prebuild method

        info!("Creating master...");
        self.create_master()?;

        let workers = master.get_workers();
        info!("Creating workers...");
        self.create_workers(&workers)?;
        info!("Writing to master/master.cfg...");
        self.write_master_config(&master)?;
        info!("Writing to worker configs...");
        self.write_worker_configs(&workers)?;
        info!("Next, run the `start` subcommand to execute the master and the workers");
        Ok(())
    }

    /// This kills the buildbot workers and all instances of python and python3
    fn stop(&mut self) -> Result<(), String> {
        info!("Killing python...");
        Cmd::new("killall").arg("python").run();
        info!("Killing python3...");
        Cmd::new("killall").arg("python3").run();
        info!("Killing workers...");
        Cmd::new("killall").arg("buildbot-worker").run();
        Ok(())
    }

    /// This starts the master and the workers
    fn start(&mut self, workers: &[Worker]) -> Result<(), String> {
        info!("Starting workers and masters...");
        self.start_master()?;
        self.start_workers(workers)?;
        Ok(())
    }

    fn install_python(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn install_buildbot(&mut self) -> Result<(), String> {
        Ok(())
    }
}

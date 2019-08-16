use crate::{yes_or_no, Cmd, File, MasterConfig, Worker, AUTH_TOKEN_PATH};

use std::path::PathBuf;
use std::process::exit;

/// This trait describes how to build rusty-ci using a particular backend.
/// For example, if you dont want to directly install the rusty-ci dependencies
/// using the Cmd object as this trait defaults to, you can implement the trait
/// for your type, and change the install method, similar to the BashBuildSystem
/// implementation in this module.
pub trait BuildSystem {
    /// Preinstall is called by the install method unless it is overloaded.
    /// This is usefult for printing a warning message or prompting the user before
    /// installing the dependencies for rusty-ci
    fn preinstall(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// This method installs rusty-ci's dependencies, python3 and buildbot.
    fn install(&mut self) -> Result<(), String> {
        self.preinstall()?; // Call the preinstall method

        info!("Installing Python...");
        self.install_python()?;
        info!("Installing Buildbot...");
        self.install_buildbot()?;
        info!(
            "Next, write your VCS's api token to '{}', and then run the `build` subcommand",
            AUTH_TOKEN_PATH
        );
        Ok(())
    }

    /// This method is similar to preinstall, but it is called by the build
    /// method instead of the install method
    fn prebuild(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Rebuild master without killing any running processes
    fn rebuild(&mut self, master: MasterConfig) -> Result<(), String> {
        self.prebuild()?; // Call the prebuild method

        if !yes_or_no("Have you already run the install subcommand and activated your virtual env in this shell? (y/n) ") {
            error!("You must run the install subcommand and activate your venv before the running the build subcommand!");
            exit(0);
        }

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
        info!("Starting workers...");
        self.start_workers(&workers)?;
        Ok(())
    }

    fn build(&mut self, master: MasterConfig) -> Result<(), String> {
        self.prebuild()?; // Call the prebuild method

        if !yes_or_no("Have you already run the install subcommand and activated your virtual env in this shell? (y/n) ") {
            error!("You must run the install subcommand and activate your venv before the running the build subcommand!");
            exit(0);
        }

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

    /// This starts the master and the workers
    fn start(&mut self, workers: &[Worker]) -> Result<(), String> {
        if !yes_or_no("Have you already run the build subcommand and activated your virtual env in this shell? (y/n) ") {
            error!("You must run the build subcommand and activate your venv before the running the start subcommand!");
            exit(0);
        }

        info!("Starting workers and masters...");
        self.start_master()?;
        self.start_workers(workers)?;
        Ok(())
    }

    /// This kills the buildbot workers and all instances of python and python3
    fn stop(&mut self) -> Result<(), String> {
        if !yes_or_no("Are you sure you want to kill rusty-ci? This will also kill all python processes! (y/n) ") {
            error!("You weren't sure!");
            exit(0);
        }

        info!("Killing python...");
        Cmd::new("killall").arg("python").run();
        info!("Killing python3...");
        Cmd::new("killall").arg("python3").run();
        info!("Killing workers...");
        Cmd::new("killall").arg("buildbot-worker").run();
        Ok(())
    }

    /// This method is used by the `rebuild` method to update the master without killing it
    fn reconfigure_master(&mut self) -> Result<(), String> {
        let buildbot = |sub_command| -> Result<(), String> {
            Cmd::new("buildbot").arg(sub_command).arg("master").run();
            Ok(())
        };

        buildbot("reconfig")?;
        buildbot("cleanupdb")?;
        Ok(())
    }

    /// This method is used by the `start` method to spin up the master
    fn start_master(&mut self) -> Result<(), String> {
        let buildbot = |sub_command| -> Result<(), String> {
            Cmd::new("buildbot").arg(sub_command).arg("master").run();
            Ok(())
        };

        buildbot("stop")?;
        buildbot("reconfig")?;
        buildbot("cleanupdb")?;
        buildbot("start")?;
        Ok(())
    }

    /// This method is used by the `start` method to spin up the workers
    fn start_workers(&mut self, workers: &[Worker]) -> Result<(), String> {
        let start_worker = |dir| -> Result<(), String> {
            Cmd::new("buildbot-worker").arg("restart").arg(&dir).run();
            Ok(())
        };

        for worker in workers {
            start_worker(worker.get_dir())?;
        }

        Ok(())
    }

    /// Creates each worker in its proper directory
    fn create_workers(&mut self, workers: &[Worker]) -> Result<(), String> {
        let make_worker = |dir| -> Result<(), String> {
            Cmd::new("buildbot-worker")
                .arg("create-worker")
                .arg(&dir)
                .arg("localhost")
                .arg(&dir)
                .arg("pass")
                .run();

            Ok(())
        };

        for worker in workers {
            make_worker(worker.get_dir())?;
        }

        Ok(())
    }

    /// Writes the configuration `buildbot.tac` file for each worker
    fn write_worker_configs(&mut self, workers: &[Worker]) -> Result<(), String> {
        for worker in workers {
            let mut path = PathBuf::new();
            path.push(worker.get_dir());
            path.push("buildbot.tac");

            match File::write(path, worker.to_string()) {
                Err(e) => Err(e + &format!(
                    "\nDid you enter a valid basedir for the \"{}\" worker?",
                    worker.get_name()
                )),
                Ok(()) => Ok(()),
            }?;
        }

        Ok(())
    }

    /// Creates the master in the `master` directory
    fn create_master(&mut self) -> Result<(), String> {
        Cmd::new("buildbot")
            .arg("create-master")
            .arg("master")
            .run();
        Ok(())
    }

    /// Writes the master configuration file
    fn write_master_config(&mut self, master: &MasterConfig) -> Result<(), String> {
        match File::write("master/master.cfg", master.to_string()) {
            Err(e) => Err(e + "\nDid you enter your venv by running `. venv/bin/activate`?"),
            Ok(()) => Ok(()),
        }
    }

    /// This method installs Python.
    /// You probably do need to overload this, I dont know
    /// for sure if it completely works. The bash impl of the buildsystem
    /// is the most reliable for now.
    fn install_python(&mut self) -> Result<(), String> {
        let apt_install = |package| -> Result<(), String> {
            Cmd::new("apt-get")
                .arg("install")
                .arg(package)
                .arg("-y")
                .run();
            Ok(())
        };

        apt_install("python3-dev")?;
        apt_install("python3-pip")?;
        apt_install("python3-venv")?;
        Ok(())
    }

    /// This method installs buildbot. Again, you probably want to overload this
    /// because it does not use a Python virtual environment, or `venv`.
    /// The `venv` is important because it does not modify the system wide packages.
    fn install_buildbot(&mut self) -> Result<(), String> {
        Cmd::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("-U")
            .arg("pip")
            .run();

        Cmd::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("-U")
            .arg("buildbot[bundle]")
            .run();

        Cmd::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("-U")
            .arg("txrequest")
            .run();

        Cmd::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("-U")
            .arg("treq")
            .run();

        Cmd::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("buildbot-worker")
            .arg("setuptools-trial")
            .run();

        Ok(())
    }
}

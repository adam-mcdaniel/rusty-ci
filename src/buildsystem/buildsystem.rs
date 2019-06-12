use crate::{MasterConfig, Worker};
use std::str::from_utf8;
use std::process::Command;
use std::path::PathBuf;
use std::fs::OpenOptions;

use std::io::prelude::*;



pub trait BuildSystem {
    fn install(&self) -> Result<(), String> {
        println!("Installing Python...");
        self.install_python()?;
        println!("Installing Buildbot...");
        self.install_buildbot()?;

        Ok(())
    }

    fn build(&self, master: MasterConfig, workers: Vec<Worker>) -> Result<(), String> {
        
        println!("Creating master...");
        self.create_master()?;
        
        println!("Creating workers...");
        self.create_workers(&workers)?;
        println!("Writing to master/master.cfg...");
        self.write_master_config(&master)?;
        println!("Writing to worker configs...");
        self.write_worker_configs(&workers)?;
        println!("Starting workers and masters...");
        self.start(&workers)?;

        Ok(())
    }

    fn start(&self, workers: &Vec<Worker>) -> Result<(), String> {
        self.start_master()?;
        self.start_workers(workers)?;
        Ok(())
    }


    fn start_master(&self) -> Result<(), String> {
        let buildbot = |sub_command| {
            match Command::new("buildbot")
                    .arg(sub_command)
                    .arg("master")
                    .output() {
                // Ok(_) => Ok(()),
                Ok(o) => {
                    println!("{}", from_utf8(&o.stdout).unwrap());
                    Ok(())
                },
                Err(e) => Err(format!("Failed to configure master because {}", e.to_string()))
            }
        };

        buildbot("stop")?;
        buildbot("reconfig")?;
        buildbot("cleanupdb")?;
        buildbot("start")?;
        
        Ok(())
    }


    fn start_workers(&self, workers: &Vec<Worker>) -> Result<(), String> {
        let start_worker = |dir| {
            match Command::new("buildbot")
                    .arg("restart")
                    .arg(&dir)
                    .output() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to start worker at {} because {}", &dir, e.to_string()))
            }
        };

        for worker in workers {
            start_worker(worker.get_dir())?;
        }
        
        Ok(())
    }

    fn create_workers(&self, workers: &Vec<Worker>) -> Result<(), String> {
        let make_worker = |dir| {
            match Command::new("buildbot-worker")
                    .arg("create-worker")
                    .arg(&dir)
                    .arg("localhost")
                    .arg(&dir)
                    .arg("pass")
                    .output() {
                // Ok(o) => {
                //     println!("{}", from_utf8(&o.stdout).unwrap());
                //     Ok(())
                // },
                Ok(o) => Ok(()),
                Err(e) => Err(format!("Failed to create worker at {} because {}", dir, e.to_string()))
            }
        };

        for worker in workers {
            make_worker(worker.get_dir())?;
        }

        Ok(())
    }


    fn write_worker_configs(&self, workers: &Vec<Worker>) -> Result<(), String> {
        for worker in workers {
            let mut path = PathBuf::new();
            path.push(worker.get_dir());
            path.push("buildbot.tac");

            let mut worker_cfg = OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .open(&path);
            match &mut worker_cfg {
                Ok(file) => {
                    match writeln!(file, "{}", worker.to_string()) {
                        Ok(_) => {Ok(())}, 
                        Err(e) => Err(format!("Failed to write worker config to {} because {}", path.display(), e.to_string()))
                    }?;
                    Ok(())
                },
                Err(e) => Err(format!("Failed to write worker config to {} because {}", path.display(), e.to_string()))
            }?;
        }

        Ok(())
    }

    fn create_master(&self) -> Result<(), String> {
        match Command::new("buildbot")
                .arg("create-master")
                .arg("master")
                .output() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create master because {}", e.to_string()))
        }
    }

    fn write_master_config(&self, master: &MasterConfig) -> Result<(), String> {
        let mut master_cfg = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open("master/master.cfg");
        match &mut master_cfg {
            Ok(file) => {
                match writeln!(file, "{}", master.to_string()) {
                    Ok(_) => {Ok(())}, 
                    Err(e) => Err(format!("Failed to write to master.cfg because {}", e.to_string()))
                }?;
                Ok(())
            },
            Err(e) => Err(format!("Failed to write to master.cfg because {}", e.to_string()))
        }
    }


    fn install_python(&self) -> Result<(), String> {
        let apt_install = |package| {
            match Command::new("apt-get")
                    .arg("install")
                    .arg(package)
                    .arg("-y")
                    .output() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to apt install {} because {}", package, e.to_string()))
            }
        };

        apt_install("python3-dev")?;
        apt_install("python3-pip")?;
        apt_install("python3-venv")?;
        Ok(())
    }

    fn install_buildbot(&self) -> Result<(), String> {
        match Command::new("python3")
                .arg("-m")
                .arg("pip")
                .arg("install")
                .arg("-U")
                .arg("pip")
                .output() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to upgrade pip because {}", e.to_string()))
        }?;
        
        match Command::new("python3")
                .arg("-m")
                .arg("pip")
                .arg("install")
                .arg("'buildbot[bundle]'")
                .output() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to pip install 'buildbot[bundle]' because {}", e.to_string()))
        }?;
        
        match Command::new("python3")
                .arg("-m")
                .arg("pip")
                .arg("install")
                .arg("buildbot-worker")
                .arg("setuptools-trial")
                .output() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to pip install 'buildbot-worker' and 'setuptools-trial' because {}", e.to_string()))
        }?;
    
        Ok(())
    }
}
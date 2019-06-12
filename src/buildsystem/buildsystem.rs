use crate::{MasterConfig, Worker, File, Cmd};
use std::path::PathBuf;



pub trait BuildSystem {
    fn install(&self) -> Result<(), String> {
        info!("Installing Python...");
        self.install_python()?;
        info!("Installing Buildbot...");
        self.install_buildbot()?;

        Ok(())
    }

    fn build(&self, master: MasterConfig, workers: Vec<Worker>) -> Result<(), String> {
        info!("Creating master...");
        self.create_master()?;
        info!("Creating workers...");
        self.create_workers(&workers)?;
        info!("Writing to master/master.cfg...");
        self.write_master_config(&master)?;
        info!("Writing to worker configs...");
        self.write_worker_configs(&workers)?;
        info!("Starting workers and masters...");
        self.start(&workers)?;
        Ok(())
    }

    fn start(&self, workers: &Vec<Worker>) -> Result<(), String> {
        self.start_master()?;
        self.start_workers(workers)?;
        Ok(())
    }

    fn start_master(&self) -> Result<(), String> {
        let buildbot = |sub_command| -> Result<(), String> {
            Cmd::new("buildbot")
                .arg(sub_command)
                .arg("master")
                .run();
            Ok(())
        };

        buildbot("stop")?;
        buildbot("reconfig")?;
        buildbot("cleanupdb")?;
        buildbot("start")?;
        Ok(())
    }


    fn start_workers(&self, workers: &Vec<Worker>) -> Result<(), String> {
        let start_worker = |dir| -> Result<(), String> {
            Cmd::new("buildbot-worker")
                    .arg("restart")
                    .arg(&dir)
                    .run();
            Ok(())
        };

        for worker in workers {
            start_worker(worker.get_dir())?;
        }
        
        Ok(())
    }

    fn create_workers(&self, workers: &Vec<Worker>) -> Result<(), String> {
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


    fn write_worker_configs(&self, workers: &Vec<Worker>) -> Result<(), String> {
        for worker in workers {
            let mut path = PathBuf::new();
            path.push(worker.get_dir());
            path.push("buildbot.tac");

            File::write(path, worker.to_string())?;
        }

        Ok(())
    }

    fn create_master(&self) -> Result<(), String> {
        Cmd::new("buildbot")
                .arg("create-master")
                .arg("master")
                .run();
        Ok(())
    }

    fn write_master_config(&self, master: &MasterConfig) -> Result<(), String> {
        File::write("master/master.cfg", master.to_string())?;
        Ok(())
    }


    fn install_python(&self) -> Result<(), String> {
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

    fn install_buildbot(&self) -> Result<(), String> {
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
                .arg("install")
                .arg("'buildbot[bundle]'")
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
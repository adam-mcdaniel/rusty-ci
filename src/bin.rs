#[macro_use]
extern crate rusty_ci;

use rusty_ci::File;
use rusty_ci::{BuildSystem, BashBuildSystem, MasterConfig, Worker};
use rusty_yaml::Yaml;
use std::process::exit;
use clap::{clap_app, crate_version, AppSettings};


fn install(mut b: impl BuildSystem) {
    match b.install() {
        Ok(_) => {
            println!("Successfully finished install");
        },
        Err(e) => {
            println!("There was a problem while installing: {}", e);
        }
    };
}


fn build(mut b: impl BuildSystem, yaml: Yaml) {
    let mut workers = vec![];
    let workers_section = match yaml.get_section("workers") {
        Ok(w) => w,
        Err(e) => {
            error!("There was a problem reading the yaml file: {}", e);
            exit(1);
        }
    };
    for worker in workers_section {
        workers.push(Worker::from(worker));
    }

    let master = MasterConfig::from(yaml);

    match b.build(master, workers) {
        Ok(_) => {},
        Err(e) => {
            error!("There was a problem while building: {}", e);
        }
    };

    println!("Successfully finished build");
}


fn main() {

    let buildsystem = BashBuildSystem::new();


    let matches = clap_app!(rusty_ci =>
        (version: crate_version!())
                    (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                    (about: "A continuous integration tool written in Rust")
                    (@subcommand install =>
                        (about: "Install python3 and buildbot")
                        (version: "0.0.1")
                        (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                    )
                    (@subcommand build =>
                        (about: "Build and launch rusty-ci from an input yaml file")
                        (version: "0.0.1")
                        (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                        (@arg YAML: +required "The path to the YAML file")
                    )
        ).setting(AppSettings::ArgRequiredElseHelp)
            .get_matches();
    

    match matches.subcommand_name() {
        Some("install") => {
            info!("Installing dependencies for rusty-ci...");
            install(buildsystem)
        },
        Some("build") => {
            let yaml_path = matches.subcommand_matches("build").unwrap().value_of("YAML").unwrap();
            info!("Building rusty-ci from {}...", &yaml_path);
            let content = match File::read(yaml_path) {
                Ok(s) => s,
                Err(e) => {
                    error!("There was a problem reading {}: {}", yaml_path, e.to_string());
                    exit(1);
                }
            };
            let yaml = Yaml::from(content);
            build(buildsystem, yaml)
        },
        _ => {}
    }
}

#[macro_use]
extern crate rusty_ci;

use rusty_ci::File;
use rusty_ci::{BuildSystem, DefaultBuildSystem, Bash, Makefile, MasterConfig, Worker};
use rusty_yaml::Yaml;
use std::process::exit;
use clap::{clap_app, crate_version, AppSettings};


fn main() {
    let matches = clap_app!(rusty_ci =>
        (version: crate_version!())
                    (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                    (about: "A continuous integration tool written in Rust")
                    (@group BUILDSYSTEM =>
                        (@arg bash: -b --bash "Uses bash to install and build rusty-ci's output")
                        (@arg make: -m --make "Uses make to install and build rusty-ci's output")
                    )
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
    

    let buildsystem: Box<dyn BuildSystem>;
    if matches.is_present("bash") {
        buildsystem = Box::new(Bash::new());
    } else if matches.is_present("make") {
        buildsystem = Box::new(Makefile::new());
    } else {
        buildsystem = Box::new(DefaultBuildSystem::new());
    }

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





/// This method takes a boxed buildsystem and runs its install routine
fn install(mut b: Box<dyn BuildSystem>) {
    match b.install() {
        Ok(_) => {
            println!("Successfully finished install");
        },
        Err(e) => {
            println!("There was a problem while installing: {}", e);
        }
    };
}

/// This function takes a boxed BuildSystem trait object and uses it
/// to run the `build` method on the object with the proper data.
/// It constructs the workers and the master config file from an input yaml,
/// and feeds it to the buildsystem.
fn build(mut b: Box<dyn BuildSystem>, yaml: Yaml) {
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
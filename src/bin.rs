// use rusty_ci::Makefile;
use rusty_ci::{BuildSystem, DefaultBuildSystem, BashBuildSystem, MasterConfig, Worker};
use rusty_yaml::Yaml;
use std::io::{self, Read};
use std::fs::File;
use std::process::exit;
use clap::{clap_app, crate_version, AppSettings};


fn install(b: impl BuildSystem) {
    match b.install() {
        Ok(_) => {},
        Err(e) => {
            println!("There was a problem while installing: {}", e);
        }
    };
}


fn build(b: impl BuildSystem, yaml: Yaml) {
    let mut workers = vec![];
    let workers_section = match yaml.get_section("workers") {
        Ok(w) => w,
        Err(e) => {
            println!("There was a reading the yaml file: {}", e);
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
            println!("There was a problem while building: {}", e);
        }
    };
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
        Some("install") => install(buildsystem),
        Some("build") => {
            let yaml_path = matches.subcommand_matches("build").unwrap().value_of("YAML").unwrap();

            match &mut File::open(&yaml_path) {
                Ok(f) => {
                    let mut content = String::from("");
                    match f.read_to_string(&mut content) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("There was a problem reading {}: {}", yaml_path, e.to_string());
                        }
                    };
                    let yaml = Yaml::from(content);
                    println!("{}", yaml);
                    build(buildsystem, yaml)
                },
                Err(e) => {
                    println!("There was a problem trying to open {}: {}", yaml_path, e.to_string());
                }
            }
        },
        _ => {}
    }

    // let mut stdin = String::new();
    // io::stdin().read_to_string(&mut stdin)?;
    // if stdin.len() == 0 {
    //     return Ok(());
    // }
    // let yaml = Yaml::from(stdin);
    // println!("{}", Makefile::from(yaml));
}

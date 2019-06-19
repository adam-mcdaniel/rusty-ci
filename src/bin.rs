#[macro_use]
extern crate rusty_ci;

use clap::{clap_app, crate_version, AppSettings};
use rusty_ci::{input, yes_or_no, File};
use rusty_ci::{Bash, BuildSystem, Makefile, MasterConfig, Worker};
use rusty_yaml::Yaml;
use std::process::exit;


fn main() {
  let matches = clap_app!(rusty_ci =>
  (version: crate_version!())
              (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
              (about: "A continuous integration tool written in Rust")
              (@subcommand install =>
                  (about: "Install buildbot")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@group BUILDSYSTEM =>
                      (@arg bash: -b --bash "Uses bash to install and build rusty-ci's output")
                      (@arg make: -m --make "Uses make to install and build rusty-ci's output")
                  )
              )
              (@subcommand build =>
                  (about: "Build rusty-ci from an input yaml file")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@arg YAML: +required "The path to the YAML file")
                  // We can add support for different build systems for building in the future
                  // (@group BUILDSYSTEM =>
                  //     (@arg bash: -b --bash "Uses bash to install and build rusty-ci's output")
                  //     (@arg make: -m --make "Uses make to install and build rusty-ci's output")
                  // )
              )
              (@subcommand start =>
                  (about: "Launch rusty-ci from an input yaml file")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@arg YAML: +required "The path to the YAML file")
              )
              (@subcommand setup =>
                  (about: "Output a template YAML file for you to change to customize")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
              )
  )
  .setting(AppSettings::ArgRequiredElseHelp)
  .get_matches();

  // Figure out the proper backend buildsystem to use
  let buildsystem: Box<dyn BuildSystem> = match matches.subcommand_name() {
    Some(subcommand) => {
      let sub_matches = matches
        .subcommand_matches(subcommand)
        .unwrap();
      if sub_matches.is_present("bash") {
        Box::new(Bash::new())
      } else if sub_matches.is_present("make") {
        Box::new(Makefile::new())
      } else {
        // Default is bash
        Box::new(Bash::new())
      }
    },
    // Default is bash
    None => Box::new(Bash::new())
  };


  match matches.subcommand_name() {
    Some("install") => {
      info!("Installing dependencies for rusty-ci...");
      install(buildsystem);
    }
    Some("build") => {
      let yaml_path = matches
        .subcommand_matches("build")
        .unwrap()
        .value_of("YAML")
        .unwrap();

      info!("Building rusty-ci from {}...", &yaml_path);
      let content = match File::read(yaml_path) {
        Ok(s) => s,
        Err(e) => {
          error!(
            "There was a problem reading {}: {}",
            yaml_path,
            e.to_string()
          );
          exit(1);
        }
      };
      let yaml = Yaml::from(content);
      build(buildsystem, yaml)
    }
    Some("start") => {
      let yaml_path = matches
        .subcommand_matches("start")
        .unwrap()
        .value_of("YAML")
        .unwrap();
      info!("Starting workers and master from {}...", &yaml_path);
      let content = match File::read(yaml_path) {
        Ok(s) => s,
        Err(e) => {
          error!(
            "There was a problem reading {}: {}",
            yaml_path,
            e.to_string()
          );
          exit(1);
        }
      };
      let yaml = Yaml::from(content);
      start(buildsystem, yaml)
    }
    Some("setup") => {
      match setup() {
        Ok(_) => {},
        Err(e) => {
          error!("There was a problem writing the template yaml file: {}", e);
          exit(1);
        }
      };
      info!("Next, run the `install` subcommand command using either the `bash` or `make` flag");
    }
    _ => {}
  }
}

/// This function writes a template YAML file for the user to edit as needed.
fn setup() -> Result<(), String> {
  let filename = input("Where do you want the output template yaml to be? ");
  if yes_or_no("Are you sure? (y/n) ") {
    info!("Writing template yaml file to {}...", filename);
    File::write(
      filename,
      r#"
# This section holds data specific to the master of the workers
master:
  # The title subsection of the master holds the title of your web gui
  title: "Rusty-CI"
  title-url: "https://github.com/adam-mcdaniel/rusty-ci"

  # This is the ip of the web-gui
  # The port is 8010
  webserver-ip: localhost

  # The address of your repository
  repo: "https://github.com/adam-mcdaniel/rusty-ci"

  # The number of seconds to wait before checking for updates on your repository
  # Two minutes is a good poll interval
  poll-interval: 120

# This section holds data specific to the handler that will look for
# pull requests / merge requests on your repository
merge-request-handler:
  # This is basically the website you're using for version control
  # Right now, github is the only supported site
  # If you're using an unsupported version control system, no worries,
  # rusty-ci just wont run on pull requests.
  version-control-system: github
  # The username of the owner of the repository
  owner: adam-mcdaniel

  # The name of the repository
  repo-name: rusty-ci

  # You dont want to run arbitrary code on your machine when anyone
  # makes a pull request. Rusty-CI will not test anyone's pull request
  # if their username is not in this list.
  whitelist:
    - adam-mcdaniel

  # The password a whitelisted user can comment on a merge / pull request
  # to mark it for testing; that is if the pull request was made by a non-whitelisted
  # user. If the pull request was made by a whitelisted user, it is automatically run.
  password: "ok to test"


# This section holds each worker
# You can have as many workers as youd like, just be sure to fill out
# each of their fields out properly.
workers:
  # The name of this worker is `test-worker`
  test-worker:
    # The ip of the master
    masterhost: localhost
    # The port of the master
    # This is not the same as the web gui port!
    masterport: 9989
    # The absolute path to the working directory of this worker
    basedir: '/home/adam/Desktop/rusty-ci/testing/test-worker'
    # The password for this worker
    # This is used by the master to give the worker a job
    password: pass

# This section holds each scheduler.
# Like the workers section, you may have as many schedulers as youd like.
schedulers:
  # Create a scheduler named `ci-change`
  # This scheduler will trigger the `rusty-ci-test` builder whenever it
  # detects a change in a yaml file for any branch.
  ci-change:
    # This is a regular expression that matches a branch.
    # If there is a change in a branch whos name matches this regex,
    # it will be checked by the following triggers section.
    # THIS WILL ONLY USE THE FIRST REGULAR EXPRESSION IN THIS SECTION TO MATCH THE BRANCH
    branch: ".*"
    # If a change has occurred in a branch that matches the regex in the branch section,
    # Then the files that were changed are matched against the regular expressions in the
    # triggers section. You can have any number of regular expressions in the triggers section.
    # If any one of them matches the name of a file that was changed in a matched branch,
    # then the builders in this scheduler's `builders` section are executed.
    triggers:
      - '.*\.yaml'
      - '.*\.sh'
      - '.*Makefile'
    # This scheduler triggers the `rusty-ci-test` builder.
    # You can put as many builders as youd like here, and the scheduler will start them all.
    builders:
      - rusty-ci-test

# These are the builders that are executed by the schedulers
# Each has its own specific task that is delegated to one or more workers
# When a builder is run, its script is run on the command line.
# You can have as many builders as youd like as well.
builders:
  # The name of the builder is `rusty-ci-test`
  rusty-ci-test:
    # This is the shell script that the workers will run when this builder is executed
    # You can have as many instructions as youd like
    # Mind you, you cannot use the |, >, <, >>, <<... operators. Sadly, buildbot
    # passes each item separated by whitespace as another parameter to function.
    script:
      - echo Hello world!
      - echo Im an instruction in a script!
    # These are the workers to delegate this build job to
    workers:
      - test-worker
    # The repo to refresh from before running
    repo: "https://github.com/adam-mcdaniel/rusty-ci"
"#,
    )?;
    info!("All done!");
  } else {
    error!("You weren't sure!");
  }

  Ok(())
}


/// This method takes a boxed BuildSystem trait object and runs its install routine
fn start(mut b: Box<dyn BuildSystem>, yaml: Yaml) {
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

  match b.start(&workers) {
    Ok(_) => {
      println!("Successfully started workers and master");
    }
    Err(e) => {
      println!("There was a problem while starting: {}", e);
    }
  };
}


/// This method takes a boxed BuildSystem trait object and runs its install routine
fn install(mut b: Box<dyn BuildSystem>) {
  match b.install() {
    Ok(_) => {
      println!("Successfully finished install");
    }
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
    Ok(_) => {}
    Err(e) => {
      error!("There was a problem while building: {}", e);
    }
  };

  println!("Successfully finished build");
}
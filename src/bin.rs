#[macro_use]
extern crate rusty_ci;

use clap::{clap_app, crate_version, AppSettings, Arg, SubCommand};
use rusty_ci::{File, unwrap};
use rusty_ci::{Bash, Quiet, BuildSystem, MailNotifier, Makefile, MasterConfig, Worker};
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
                  (@arg quiet: -q --quiet "Don't ask user anything")
              )
              (@subcommand start =>
                  (about: "Launch rusty-ci from an input yaml file")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@arg quiet: -q --quiet "Don't ask user anything")
                  (@arg MASTER_YAML: +required "The path to the YAML file")
              )
              (@subcommand setup =>
                  (about: "Output a template YAML file for you to change to customize")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@arg MASTER_YAML: +takes_value default_value("template.yaml") "The path to write the master YAML file")
                  (@arg MAIL_YAML: +takes_value default_value("mail.yaml") "The path to write the mail list YAML file")
              )
              (@subcommand stop =>
                  (about: "Stop rusty-ci")
                  (version: "0.1.0")
                  (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                  (@arg quiet: -q --quiet "Don't ask user anything")
              )
  )
  .subcommand(
    SubCommand::with_name("build")
        .about("Build rusty-ci from an input yaml file")
        .version("0.1.0")
        .author("Adam McDaniel <adam.mcdaniel17@gmail.com>")
        .arg(
          Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .takes_value(false)
            .help("Don't ask user anything")
        )
        .arg(
          Arg::with_name("MAIL_YAML")
            .short("m")
            .long("mail")
            .takes_value(true)
            .help("The path to the YAML file dedicated to SMTP authentication info for sending email notifications")
        )
        .arg(
          Arg::with_name("MASTER_YAML")
            .required(true)
            .help("The path to the YAML file")
        )
        .setting(AppSettings::ArgRequiredElseHelp)
  )
  .setting(AppSettings::ArgRequiredElseHelp)
  .after_help("To start a project, run the `setup` subcommand.\nBe sure to follow the instructions after each subcommand very carefully!")
  .get_matches();


  // Figure out the proper backend buildsystem to use
  let mut buildsystem: Box<dyn BuildSystem> = match matches.subcommand_name() {
    Some(subcommand) => {
      let sub_matches = matches.subcommand_matches(subcommand).unwrap();
      if sub_matches.is_present("bash") {
        Box::new(Bash::new())
      } else if sub_matches.is_present("make") {
        Box::new(Makefile::new())
      } else if sub_matches.is_present("quiet") {
        Box::new(Quiet::new())
      } else {
        // Default is bash
        Box::new(Bash::new())
      }
    }
    // Default is bash
    None => Box::new(Bash::new()),
  };

  match matches.subcommand_name() {
    Some("stop") => {
      info!("Stopping Rusty-CI...");
      match buildsystem.stop() {
        Err(e) => {
          error!("There was a problem stopping Rusty-CI: {}", e);
          exit(1);
        }
        Ok(()) => {},
      };
    }
    Some("install") => {
      info!("Installing dependencies for rusty-ci...");
      install(buildsystem);
    }
    Some("build") => {
      let yaml_path = matches
        .subcommand_matches("build")
        .unwrap()
        .value_of("MASTER_YAML")
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
      let master_yaml = Yaml::from(content);

      // If the MAIL_YAML argument is passed, open the file with that path
      // and return a yaml object from its contents
      let mail_yaml = match matches
        .subcommand_matches("build")
        .unwrap()
        .value_of("MAIL_YAML")
      {
        Some(mail_yaml) => match File::read(mail_yaml) {
          Ok(s) => Some(Yaml::from(s)),
          Err(e) => {
            error!(
              "There was a problem reading {}: {}",
              mail_yaml,
              e.to_string()
            );
            exit(1);
          }
        },
        None => None,
      };
      build(buildsystem, master_yaml, mail_yaml)
    }
    Some("start") => {
      let yaml_path = matches
        .subcommand_matches("start")
        .unwrap()
        .value_of("MASTER_YAML")
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
      start(buildsystem, yaml);
    }
    Some("setup") => {
      let master_path = matches.subcommand_matches("setup").unwrap().value_of("MASTER_YAML").unwrap();
      let mail_path = matches.subcommand_matches("setup").unwrap().value_of("MAIL_YAML").unwrap();
      match setup(master_path, mail_path) {
        Ok(_) => {}
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
fn setup(master_filename: &str, mail_filename: &str) -> Result<(), String> {
    info!("Writing template master yaml file to {}...", master_filename);
    File::write(
      master_filename,
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
    # The worker files will be installed in this directory
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
    # This scheduler triggers the `rusty-ci-test` builder.
    # You can put as many builders as youd like here, and the scheduler will start them all.
    builders:
      - rusty-ci-test

    # This will make the current scheduler run if the "your-scheduler-name-here"
    # has run successfully. You can only put one scheduler name in this section.
    # depends: "your-scheduler-name-here"
    # IF YOU USE THE `depends` SECTION, YOU SHOULD REMOVE OR COMMENT THE FOLLOWING SECTIONS
    # Using the depends section will ignore the `branch`, `triggers`, and `password` sections

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
      - ".*Makefile"
    # The password a whitelisted user can comment on a merge / pull request
    # to mark it for testing; that is if the pull request was made by a non-whitelisted
    # user. If the pull request was made by a whitelisted user, it is automatically run.
    password: "ok to test"

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

  info!("Writing template mail yaml file to {}...", mail_filename);
  File::write(
    mail_filename,
    r#"# Rusty-CI will automatically email "interested users" about
# all tests that run. The list of "interested users" is the
# list of people who have a commit in the branch or pull request.

# The extra recipients to email
extra-recipients:
  # Emails under the failure section will be emailed
  # info about every failed build
  failure:
    - failure@gmail.com
  # Emails under the success section will be emailed
  # info about every successful build
  success:
    - success@gmail.com
  # Emails under the all section will be emailed
  # info about every build
  all:
    - all_tests@gmail.com


# The "from" email address used to send email updates to recipients
from-address: your-email-here@gmail.com

# The suffix to add to the interested users' usernames
# to get an email we can send updates to.
lookup: gmail.com

# The smtp relay hostname (self explanatory)
# gmail's smtp relay hostname is `smtp.gmail.com`
smtp-relay-host: smtp.gmail.com

# The smtp relay port (self explanatory)
# 587 is the smtp port that `smtp.gmail.com` uses
smtp-port: 587

# The password used to login to the "from" email address account
smtp-password: "p@$$w0rd""#,
  )?;
  info!("All done!");

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
      println!("Run `tail -f master/twistd.log` to see the log output for your CI!");
      println!("Go to http://{}:8010 to view your webgui", unwrap(&yaml.get_section("master").unwrap(), "webserver-ip"))
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
fn build(mut b: Box<dyn BuildSystem>, master_yaml: Yaml, mail_yaml: Option<Yaml>) {
  let mut workers = vec![];
  let workers_section = match master_yaml.get_section("workers") {
    Ok(w) => w,
    Err(e) => {
      error!("There was a problem reading the yaml file: {}", e);
      exit(1);
    }
  };
  for worker in workers_section {
    workers.push(Worker::from(worker));
  }
  let mut master = MasterConfig::from(master_yaml);

  match mail_yaml {
    Some(mn) => master.set_mail_notifier(MailNotifier::from(mn)),
    None => {}
  }

  match b.build(master, workers) {
    Ok(_) => {
      println!("Successfully finished build");
    }
    Err(e) => {
      error!("There was a problem while building: {}", e);
    }
  };
}

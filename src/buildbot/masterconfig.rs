use std::process::exit;
use crate::{Builder, Scheduler, Worker};


/// This struct represents the configuration file for the master.
/// This file contains the Python code for the builders and the schedulers.
/// In addition, it contains some basic data such as the title for the web ui,
/// the title url for the webui, and so on.
/// 
/// For more information on how the master configuration file works,
/// see the documentation for buildbot on their website: https://buildbot.net/
use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
pub struct MasterConfig {
    title: String, //
    title_url: String,
    git_repo: String,
    webserver_ip: String,
    poll_interval: String,
    builders: Vec<Builder>,
    schedulers: Vec<Scheduler>,
    workers: Vec<Worker>,
}


/// This is impl the for MasterConfig struct.
impl MasterConfig {
    fn new(
        title: String,
        title_url: String,
        git_repo: String,
        webserver_ip: String,
        poll_interval: String,
        builders: Vec<Builder>,
        schedulers: Vec<Scheduler>,
        workers: Vec<Worker>,
    ) -> Self {
        Self {
            title,
            title_url,
            git_repo,
            webserver_ip,
            poll_interval,
            builders,
            schedulers,
            workers,
        }
    }
}

/// This impl converts a Yaml file into a MasterConfig object.
/// This is intended to take the entire input yaml file.
impl From<Yaml> for MasterConfig {
    fn from(yaml: Yaml) -> Self {
        // Verify that the yaml section contains all the necessary subsections
        for section in ["master", "workers", "builders", "schedulers"].iter() {
            if !yaml.has_section(section) {
                error!("There was an error creating the master configuration file: '{}' section was not declared", section);
                exit(1);
            }
        }

        // Get the master susbsection, the subsection holding the web gui and git information
        let master = yaml.get_section("master").unwrap();
        

        // Verify the master subsection contains all the proper data
        for section in [
            "title",
            "title-url",
            "repo",
            "webserver-ip",
            "poll-interval",
        ]
        .iter()
        {
            if !master.has_section(section) {
                error!("There was an error creating the master configuration file: The '{}' section is not specified for master", section);
                exit(1);
            }
        }

        // Get schedulers, builders, and workers from the yaml file.
        // Because we previously verified that each subsection exists, 
        // we can unwrap the result without a problem.
        let mut schedulers = vec![];
        for scheduler in yaml.get_section("schedulers").unwrap() {
            schedulers.push(Scheduler::from(scheduler));
        }

        // Because we previously verified that each subsection exists, 
        // we can unwrap the result without a problem.
        let mut builders = vec![];
        for builder in yaml.get_section("builders").unwrap() {
            builders.push(Builder::from(builder));
        }

        // Because we previously verified that each subsection exists, 
        // we can unwrap the result without a problem.
        let mut workers = vec![];
        for worker in yaml.get_section("workers").unwrap() {
            workers.push(Worker::from(worker));
        }


        // Get all the data from the master subsection
        let title: String = master.get_section("title").unwrap().nth(0).unwrap().to_string();
        let title_url: String = master.get_section("title-url").unwrap().nth(0).unwrap().to_string();
        let git_repo: String = master.get_section("repo").unwrap().nth(0).unwrap().to_string();
        let webserver_ip: String = master.get_section("webserver-ip").unwrap().nth(0).unwrap().to_string();
        let poll_interval: String = master.get_section("poll-interval").unwrap().nth(0).unwrap().to_string();

        // Return the whole master configuration file
        Self::new(
            title,
            title_url,
            git_repo,
            webserver_ip,
            poll_interval,
            builders,
            schedulers,
            workers,
        )
    }
}

/// Converts a MasterConfig instance into the Python master configuration file for buildbot
impl Display for MasterConfig {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            r#"
# -*- python -*-
# ex: set filetype=python:
import re
from buildbot.plugins import *

# This is a sample buildmaster config file. It must be installed as
# 'master.cfg' in your buildmaster's base directory.

# This is the dictionary that the buildmaster pays attention to. We also use
# a shorter alias to save typing.
c = BuildmasterConfig = {{}}

####### WORKERS

# The 'workers' list defines the set of recognized workers. Each element is
# a Worker object, specifying a unique worker name and password.  The same
# worker name and password must be configured on the worker.
c['workers'] = [{worker_info}]
c['protocols'] = {{'pb': {{'port': 9989}}}}


c['change_source'] = []
c['change_source'].append(changes.GitPoller(
        {git_repo},
        workdir='gitpoller-workdir', branches=True, # poll all branches
        pollInterval={poll_interval}))

c['schedulers'] = []
c['builders'] = []

{schedulers}
{builders}

c['services'] = []

c['title'] = "{title}"
c['titleURL'] = {title_url}

c['buildbotURL'] = "http://{webserver_ip}:8010/"

c['www'] = dict(port=8010,
                plugins=dict(waterfall_view={{}}, console_view={{}}, grid_view={{}}))

c['db'] = {{
    # This specifies what database buildbot uses to store its state.  You can leave
    # this at its default for all but the largest installations.
    'db_url' : "sqlite:///state.sqlite",
}}
        "#,
            title = self.title,
            title_url = self.title_url,
            webserver_ip = self.webserver_ip,
            git_repo = self.git_repo,
            worker_info = self
                .workers
                .iter()
                .map(|w| {
                    format!(
                        "worker.Worker(\"{}\", \"{}\")",
                        w.get_name(),
                        w.get_password()
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
            poll_interval = self.poll_interval,
            schedulers = self.schedulers
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            builders = self.builders
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}
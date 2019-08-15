use crate::{
    unmatched_quotes, unwrap, Builder, MailNotifier, MergeRequestHandler, Scheduler, Worker,
};

use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;

/// This struct represents the configuration file for the master.
/// This file contains the Python code for the builders and the schedulers.
/// In addition, it contains some basic data such as the title for the web ui,
/// the title url for the webui, and so on.
///
/// For more information on how the master configuration file works,
/// see the documentation for buildbot on their website: https://buildbot.net/
pub struct MasterConfig {
    title: String,
    title_url: String,
    git_repo: String,
    webserver_ip: String,
    webserver_port: String,
    poll_interval: String,
    mail_notifier: Option<MailNotifier>,
    merge_request_handler: MergeRequestHandler,
    builders: Vec<Builder>,
    schedulers: Vec<Scheduler>,
    workers: Vec<Worker>,
}

/// This is impl the for MasterConfig struct.
impl MasterConfig {
    pub fn set_mail_notifier(&mut self, mail_notifier: MailNotifier) {
        self.mail_notifier = Some(mail_notifier);
    }

    pub fn get_workers(&self) -> Vec<Worker> {
        self.workers.clone()
    }
}

/// This impl converts a Yaml file into a MasterConfig object.
/// This is intended to take the entire input yaml file.
impl From<Yaml> for MasterConfig {
    fn from(yaml: Yaml) -> Self {
        // Verify that the yaml file doesnt have unmatched quotes!
        if let Some(line) = unmatched_quotes(&yaml) {
            error!("There was a problem creating the master configuration file: unmatched quotes in the line '{}'", line.trim());
            exit(1);
        }

        // Verify that the yaml section contains all the necessary subsections
        for section in [
            "master",
            "workers",
            "builders",
            "schedulers",
            "merge-request-handler",
        ]
        .iter()
        {
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
            "webserver-port",
            "poll-interval",
        ]
        .iter()
        {
            if !master.has_section(section) {
                error!("There was an error creating the master configuration file: The '{}' section is not specified for master", section);
                exit(1);
            }
        }

        let merge_request_handler =
            MergeRequestHandler::from(yaml.get_section("merge-request-handler").unwrap());

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
        let title = unwrap(&master, "title");
        let title_url = unwrap(&master, "title-url");
        let git_repo = unwrap(&master, "repo");
        let webserver_ip = unwrap(&master, "webserver-ip");
        let webserver_port = unwrap(&master, "webserver-port");
        let poll_interval = unwrap(&master, "poll-interval");

        // Return the whole master configuration file

        Self {
            title,
            title_url,
            git_repo,
            webserver_ip,
            webserver_port,
            poll_interval,
            mail_notifier: None,
            merge_request_handler,
            builders,
            schedulers,
            workers,
        }
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
import json
import requests as req
from dateutil.parser import parse as dateparse
from buildbot.plugins import *
from buildbot.www.hooks.github import GitHubEventHandler

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


c['www'] = dict(port={webserver_port},
                plugins=dict(waterfall_view={{}}, console_view={{}}, grid_view={{}}))

c['change_source'] = []
c['services'] = []

{mail_notifier}

{merge_request_handler}

c['change_source'].append(changes.GitPoller(
        "{git_repo}",
        workdir='gitpoller-workdir', branches=True, # poll all branches
        pollInterval={poll_interval}))

c['schedulers'] = []
c['builders'] = []

{schedulers}
{builders}


c['title'] = "{title}"
c['titleURL'] = "{title_url}"

c['buildbotURL'] = "http://{webserver_ip}:{webserver_port}/"

c['db'] = {{
    # This specifies what database buildbot uses to store its state.  You can leave
    # this at its default for all but the largest installations.
    'db_url' : "sqlite:///state.sqlite",
}}
        "#,
            title = self.title,
            title_url = self.title_url,
            webserver_ip = self.webserver_ip,
            webserver_port = self.webserver_port,
            git_repo = self.git_repo,
            merge_request_handler = self.merge_request_handler,
            mail_notifier = match &self.mail_notifier {
                Some(mn) => mn.to_string(),
                None => "".to_string(),
            },
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
            schedulers = self
                .schedulers
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            builders = self
                .builders
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

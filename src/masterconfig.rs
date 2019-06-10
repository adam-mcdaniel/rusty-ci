use crate::{Builder, Scheduler, Worker};

use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
pub struct MasterConfig {
    title: String,
    title_url: String,
    git_repo: String,
    webserver_ip: String,
    poll_interval: String,
    builders: Vec<Builder>,
    schedulers: Vec<Scheduler>,
    workers: Vec<Worker>,
}

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


impl From<Yaml> for MasterConfig {
    fn from(yaml: Yaml) -> Self {

        for section in ["master", "workers", "builders", "schedulers"].iter() {
            assert!(
                yaml.has_section(section),
                format!("{} section not declared", section)
            )
        }

        for section in [
            "title",
            "title-url",
            "repo",
            "webserver-ip",
            "poll-interval",
        ]
        .iter()
        {
            assert!(
                yaml.get_section("master").has_section(section),
                format!("{} section not specified for master", section)
            )
        }

        let mut schedulers = vec![];
        for scheduler in yaml.get_section("schedulers") {
            schedulers.push(Scheduler::from(scheduler));
        }

        let mut builders = vec![];
        for builder in yaml.get_section("builders") {
            builders.push(Builder::from(builder));
        }

        let mut workers = vec![];
        for worker in yaml.get_section("workers") {
            workers.push(Worker::from(worker));
        }


        let title: String = yaml
            .get_section("master")
            .get_section("title")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

        let title_url: String = yaml
            .get_section("master")
            .get_section("title-url")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

        let git_repo: String = yaml
            .get_section("master")
            .get_section("repo")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

        let webserver_ip: String = yaml
            .get_section("master")
            .get_section("webserver-ip")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

        let poll_interval: String = yaml
            .get_section("master")
            .get_section("poll-interval")
            .into_iter()
            .collect::<Vec<Yaml>>()[0]
            .to_string();

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
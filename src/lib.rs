/// This crate can also be used as a library for other tools to generate buildbot projects
#[macro_use]
pub mod helper;
pub use helper::{input, unmatched_quotes, unquote, unwrap, yes_or_no, Cmd, File};

pub mod buildbot;
pub use buildbot::{
    Builder, MailNotifier, MasterConfig, MergeRequestHandler, Scheduler, Step, Worker,
    AUTH_TOKEN_PATH,
};

pub mod buildsystem;
pub use buildsystem::*;

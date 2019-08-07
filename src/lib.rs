/// This crate can also be used as a library for other tools to generate buildbot projects
#[macro_use]
pub mod helper;
pub use helper::{File, Cmd, input, yes_or_no, unwrap, unquote, unmatched_quotes};


pub mod buildbot;
pub use buildbot::{
    AUTH_TOKEN_PATH,
    Worker,
    MasterConfig,
    MergeRequestHandler,
    MailNotifier,
    Scheduler,
    Builder,
    Step,
};

pub mod buildsystem;
pub use buildsystem::*;

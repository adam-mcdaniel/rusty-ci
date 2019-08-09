use std::fmt::Display;
use std::process::Command;
use std::str::from_utf8;

/// This struct is basically identical to the std::process::Command,
/// but when it is executed, it returns the stdout of the process as a string.
#[derive(Clone)]
pub struct Cmd {
    program: String,
    args: Vec<String>,
}

impl Cmd {
    /// Create a command that will call `program`.
    /// For example, if you want to run the command
    /// `echo 'hello world!'`, you would write:
    /// Cmd::new("echo").arg("'hello world!'").run();
    pub fn new<S: Display>(program: S) -> Self {
        Self {
            program: program.to_string(),
            args: vec![],
        }
    }

    /// Give another arg to the program we're calling
    pub fn arg<S: Display>(&mut self, s: S) -> &mut Self {
        self.args.push(s.to_string());
        return self;
    }

    /// Execute the shell command we've defined
    pub fn run(&self) -> String {
        match Command::new(&self.program).args(&self.args).output() {
            // If Ok, return stdout as String
            Ok(o) => match from_utf8(&o.stdout) {
                Ok(s) => s.to_string(),
                Err(_) => String::from(""),
            },
            Err(_) => String::from(""),
        }
    }
}

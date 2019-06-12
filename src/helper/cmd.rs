use std::str::from_utf8;
use std::fmt::Display;
use std::process::Command;


#[derive(Clone)]
pub struct Cmd {
    program: String, 
    args: Vec<String>
}

impl Cmd {
    pub fn new<S: Display>(program: S) -> Self {
        Self {
            program: program.to_string(),
            args: vec![]
        }
    }

    pub fn arg<S: Display>(&mut self, s: S) -> &mut Self {
        self.args.push(s.to_string());
        return self;
    }

    pub fn run(&self) -> String {
        match Command::new(&self.program).args(&self.args).output() {
            Ok(o) => {
                match from_utf8(&o.stdout) {
                    Ok(s) => s.to_string(),
                    Err(_) => String::from("")
                }
            },
            Err(_) => String::from("")
        }
    }
}
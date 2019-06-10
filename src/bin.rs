use rusty_ci::Makefile;
use rusty_yaml::Yaml;
use std::io::{self, Read};


fn main() -> io::Result<()> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;

    if stdin.len() == 0 {
        return Ok(());
    }

    let yaml = Yaml::from(stdin);


    println!("{}", Makefile::from(yaml));

    Ok(())
}

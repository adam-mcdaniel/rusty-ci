use std::io::{Read, Write};
use std::path::Path;
use std::fmt::Display;
use std::fs;


pub struct File;


impl File {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<String, String> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(&path);
        
        match &mut file {
            Ok(f) => {
                let mut contents = String::from("");
                match f.read_to_string(&mut contents) {
                    Ok(_) => Ok(contents),
                    Err(e) => Err(
                        format!("Could not read from file '{}' because {}", 
                            match path.as_ref().to_str() {
                                Some(s) => s,
                                None => ""
                            }, e.to_string()
                        )
                    )
                }
            },
            Err(e) => Err(
                format!("Could not open file '{}' because {}", 
                    match path.as_ref().to_str() {
                        Some(s) => s,
                        None => ""
                    }, e.to_string()
                )
            )
        }
    }


    pub fn write<P: AsRef<Path>, S: Display>(path: P, contents: S) -> Result<(), String> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path);
        
        match &mut file {
            Ok(f) => {
                match writeln!(f, "{}", contents) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(
                        format!("Could not open file '{}' because {}", 
                            match path.as_ref().to_str() {
                                Some(s) => s,
                                None => ""
                            }, e.to_string()
                        )
                    )
                }
            },
            Err(e) => Err(
                format!("Could not open file '{}' because {}", 
                    match path.as_ref().to_str() {
                        Some(s) => s,
                        None => ""
                    }, e.to_string()
                )
            )
        }
    }
}
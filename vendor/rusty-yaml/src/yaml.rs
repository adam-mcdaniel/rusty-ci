use std::fmt::{Display, Error, Formatter};
use yaml_rust::{YamlEmitter, YamlLoader};


/// Yaml struct definition
#[derive(Clone, Debug, PartialEq)]
pub struct Yaml {
    name: String,
    contents: String,
}


// Implementation of yaml struct
impl Yaml {
    /// Get the name of the current section
    /// Requires immutable access
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get a section of the current yaml
    /// Requires immutable access
    pub fn get_section<S: Display>(&self, index: S) -> Result<Self, String> {
        match YamlLoader::load_from_str(&self.contents) {
            Ok(reader) => {
                // let result = if lenreader[0];
                let result = if reader.len() > 0 {
                    reader[0][index.to_string().as_str()].clone()
                } else {
                    return Err(format!(
                        "Could not get section '{}' because the parent section is empty",
                        index
                    ));
                };

                let mut s = Self::from(result);
                s.name = index.to_string();
                Ok(s)
            }

            Err(e) => {
                return Err(format!(
                    "Could not get section '{}' because {}",
                    index,
                    e.to_string()
                ));
            }
        }
    }


    /// Get the names of all the sections in the current section
    /// Requires immutable access
    pub fn get_section_names(&self) -> Result<Vec<String>, String> {
        // Iterator over section names and collect them into a vec of strings
        match YamlLoader::load_from_str(&self.contents) {
            Ok(y) => match &y[0] {
                yaml_rust::Yaml::Hash(h) => Ok(h
                    .keys()
                    .map(|k| match k {
                        yaml_rust::Yaml::String(s) => s.clone(),
                        _ => String::from(""),
                    })
                    .collect()),
                _ => Ok(vec![]),
            },
            Err(_) => {
                return Err(format!("Malformed yaml section '{}'", self.name));
            }
        }
    }

    /// Does this yaml section have a section with this name?
    /// Requires immutable access
    pub fn has_section<S: Display>(&self, index: S) -> bool {
        self.get_section_names()
            .unwrap()
            .contains(&index.to_string())
    }

    pub fn nth<N: Into<i32>>(&self, n: N) -> Self {
        let vec = self.clone().into_iter().collect::<Vec<Yaml>>();

        let index = n.into() as usize;
        if index >= vec.len() {
            Yaml::from(self.to_string())
        } else {
            vec[index].clone()
        }
    }
}

/// Converts a yaml object into an iterator
/// Iterates over members of the section
impl IntoIterator for Yaml {
    type Item = Self;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match YamlLoader::load_from_str(&self.contents) {
            Ok(y) => match &y[0] {
                // If children are sections, iterate over sections
                yaml_rust::Yaml::Hash(h) => h
                    .clone()
                    .keys()
                    .map(|k| match k {
                        yaml_rust::Yaml::String(s) => Self::from(y[0].clone()).get_section(s),
                        _ => self.get_section(""),
                    }) // This map returns a Vec<Result<Self>>
                    .filter(|y| y.is_ok()) // Filter out all Errs
                    .map(|y| y.unwrap()) // Unwrap all results
                    .collect::<Vec<Self>>(), // Collect Vec<Self>

                // If children are values, iterate over values
                yaml_rust::Yaml::Array(a) => a
                    .iter()
                    .map(|y| Self::from(y.clone()))
                    .collect::<Vec<Self>>(),

                _ => vec![Yaml::from(self.to_string())],
            },
            Err(_) => {
                // This Yaml isnt a list, so make a vector of length one of our contents
                vec![Yaml::from(self.to_string())]
            }
        }
        .into_iter()
    }
}

// Yaml object from string
impl From<String> for Yaml {
    fn from(s: String) -> Self {
        Self {
            contents: s.to_string(),
            name: String::from(""),
        }
    }
}

// Yaml object from &str (using String implementation)
impl From<&str> for Yaml {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}


// Yaml object from &str
impl From<yaml_rust::Yaml> for Yaml {
    fn from(yaml: yaml_rust::Yaml) -> Self {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        match emitter.dump(&yaml) {
            _ => {}
        };


        let lines = out_str.lines().collect::<Vec<&str>>();
        out_str = if lines.len() > 0 {
            lines[1..].join("\n").to_string()
        } else {
            "".to_string()
        };


        Self::from(out_str)
    }
}


impl From<Yaml> for String {
    fn from(yaml: Yaml) -> Self {
        format!("{}", yaml.to_string())
    }
}

// How to display a Yaml object
impl Display for Yaml {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.contents)
    }
}

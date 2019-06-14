use rusty_yaml::Yaml;

/// This function unwraps a Yaml object,
/// takes its first value, and converts it into a string,
/// and trims quotation marks.
pub fn unwrap<S: ToString>(yaml: &Yaml, section: S) -> String {
    yaml.get_section(section.to_string())
        .unwrap()
        .nth(0)
        .unwrap()
        .to_string()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}
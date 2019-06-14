use rusty_yaml::Yaml;

/// This function unwraps a Yaml object,
/// takes its first value, and converts it into a string,
/// and trims quotation marks.
pub fn unwrap<S: ToString>(yaml: &Yaml, section: S) -> String {
    let result = yaml.get_section(section.to_string())
        .unwrap()
        .nth(0)
        .unwrap()
        .to_string();

	let first = match result.chars().nth(0) {Some(v) => v, None => ' '};
	let last = match result.chars().nth(result.len()-1) {Some(v) => v, None => ' '};
	if first == last {
		match first {
			// If the first and last character are the same, and are
			// both forms of quotes, trim the outer most ones.
			'\'' | '"' => {
				result[1..result.len()-1].to_string()
			},
			_ => {
				result.to_string()
			}
		}
	} else {
		result.to_string()
	}
}
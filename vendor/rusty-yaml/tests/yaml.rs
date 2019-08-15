//
// Unit tests for Yaml operations
//

use rusty_yaml::Yaml;


// Test name related yaml functions / methods
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yaml_get_name() {
        let y = Yaml::from(
            "
        section1:
            - test
            - [testing]
            - 2
            ",
        );
        assert_eq!(y.get_section_names(), Ok(vec!["section1".to_string()]));
    }


    // Test section related yaml functions / methods
    #[test]
    fn yaml_get_sections() {
        let y = Yaml::from(
            "
        section1:
            whoa:
                - test
                - 2
                - [value, 1, \"seven\"]
            spill-the-tea:
                - tea
                - 2
                - spill
            ",
        );

        assert_eq!(
            y.get_section("section1").and_then(|y| y.get_section("whoa")),
            Yaml::from(
                "
                whoa:
                    - test
                    - 2
                    - [value, 1, \"seven\"]
                spill-the-tea:
                    - tea
                    - 2
                    - spill
                "
            )
            .get_section("whoa")
        );
    }

    // Test yaml iteration
    #[test]
    fn yaml_iteration() {
        let y = Yaml::from(
            "
        section1:
            whoa:
                - test
                - 2
                - [value, 1, \"seven\"]
            spill-the-tea:
                - tea
                - 2
                - spill
            ",
        );

        let real_sections = y
            .get_section_names().unwrap()
            .iter()
            .map(|n| y.get_section(n))
            .filter(|y| y.is_ok())
            .map(|y| y.unwrap())
            .collect::<Vec<Yaml>>();
        let mut test_sections = vec![];

        for section in y.clone() {
            test_sections.push(section.clone());
            assert!(y.has_section(section.get_name()))
        }

        assert_eq!(real_sections, test_sections);
    }

    // Test yaml nth
    #[test]
    fn yaml_nth() {
        let y = Yaml::from(
            "
        section1:
            whoa:
                - test
                - 2
                - [value, 1, \"seven\"]
            spill-the-tea:
                - tea
                - 2
                - spill
            ",
        );
        
        assert_eq!("whoa:
  - test
  - 2
  - - value
    - 1
    - seven
spill-the-tea:
  - tea
  - 2
  - spill".to_string(), y.nth(0).to_string());
    }

    // Test yaml key: value
    #[test]
    fn yaml_key_value() {
        let y = Yaml::from(
            "
        section1:
            section2: \"testing\"
            ",
        );
        
        assert_eq!("testing".to_string(), y.get_section("section1").unwrap().get_section("section2").unwrap().nth(0).to_string());
    }
}
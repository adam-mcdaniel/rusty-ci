# rusty-yaml

A rust library to parse yaml files.

## Usage

Copy and paste the following into your Cargo.toml.

```toml
[dependencies]
rusty-yaml="0.1"
```

## Examples

````rust
use rusty_yaml::Yaml;

fn main() {
    let yaml_reader = Yaml::from(
        "
builders:
    clang-format:
        worker: asgard-worker
        script:
            - ls

    build:
        worker: asgard-worker
        script:
            - mkdir build
            - cd build
            - cmake ..
            - make -j
            - ctest -j 4
",
    );

    println!(
        "section names: {:?}",
        yaml_reader
            .get_section("builders")
            .get_section("build")
            .get_section_names()
    );


    for section in yaml_reader.get_section("builders") {
        println!("```{}```", section);
    }


    println!("has builders: {}", yaml_reader.has_section("builders"));


    for section in yaml_reader.get_section("builders") {
        println!("Name: {}", section.get_name());
        for command in section.get_section("script") {
            println!("command: '{}'", command);
        }
    }
}
````

extern crate os_info;
use os_info::{get, Type};


const SUPPORTED: &[Type] = &[
    Type::Debian,
    Type::Ubuntu,
    Type::Linux,
];


const INCOMPATIBLE: &[Type] = &[
    Type::Windows,
];


fn main() {
    let os_type = get().os_type();

    if SUPPORTED.contains(&os_type) {
        println!("Rusty-CI supports {}!", os_type);
    } else if INCOMPATIBLE.contains(&os_type) {
        panic!("{} is incompatible with Rusty-CI", os_type);
    } else {
        println!("Rusty-CI may not function properly on {}", os_type);    
    }
}
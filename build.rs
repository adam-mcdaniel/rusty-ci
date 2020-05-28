// ############################
// RUSTY-CI
// ############################
//
// This script runs when this crate is compiled or installed!
//
// Right now, all this file does is confirm
// that the host isn't using Windows OS.

extern crate os_info;
use os_info::{get, Type};

/// Operating systems that Rusty-CI supports
const SUPPORTED: &[Type] = &[
    // We definitely support Ubuntu!
    Type::Ubuntu,
    // We probably support Debian!
    Type::Debian,
    // Ubuntu-based, so probably is supported.
    Type::Pop,
    // We support linux?
    Type::Linux,
];

/// Operating systems that Rusty-CI is incompatible with
const INCOMPATIBLE: &[Type] = &[
    // Windows is lame
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

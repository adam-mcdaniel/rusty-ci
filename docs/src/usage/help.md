# Help Subcommand

Rusty-CI has four subcommands, the first of which is the `help` subcommand.
To run the help subcommand, run `rusty-ci help` or alternatively `rusty-ci`.
If you dont provide any arguments to `rusty-ci`, it will print the help message by default.

```
rusty_ci 0.6.3  
Adam McDaniel <adam.mcdaniel17@gmail.com>
A continuous integration tool written in Rust

USAGE:
    rusty-ci [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Build rusty-ci from an input yaml file
    help       Prints this message or the help of the given subcommand(s)
    install    Install buildbot
    setup      Output a template YAML file for you to change to customize
    start      Launch rusty-ci from an input yaml file
```

This tells you more about the program, but it doesn't go into any depth.
Let me explain each subcommand individually.

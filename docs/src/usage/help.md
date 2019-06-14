# Help Subcommand

Rusty-CI has four subcommands, the first of which is the `help` subcommand.
To run the help subcommand, run `rusty-ci help` or alternatively `rusty-ci`.
If you dont provide any arguments to `rusty-ci`, it will print the help message by default.

```
rusty_ci 0.4.2
Adam McDaniel <adam.mcdaniel17@gmail.com>
A continuous integration tool written in Rust

USAGE:
    rusty-ci [FLAGS] [SUBCOMMAND]

FLAGS:
    -b, --bash       Uses bash to install and build rusty-ci's output
    -h, --help       Prints help information
    -m, --make       Uses make to install and build rusty-ci's output
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Build and launch rusty-ci from an input yaml file
    help       Prints this message or the help of the given subcommand(s)
    install    Install python3 and buildbot
    setup      Output a template YAML file for you to change to customize
```

This tells you more about the program, but it doesn't go into any depth.
Let me explain each subcommand individually.

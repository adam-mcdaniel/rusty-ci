# Rusty-CI Binary

After going through the installation process, continue reading here to learn more about how to use the `rusty-ci` binary.


### Help

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

## The Install Subcommand

The `install` subcommand is responsible for installing Rusty-CI's dependencies. When you run the install subcommand, it will run several shell commands in the background to install [python3](https://www.python.org/), and [buildbot](https://buildbot.net/).

That is, if you use the default buildsystem. If you do not want Rusty-CI running anything on your machine without supervision, you can choose from the other buildsystems. Right now, `Rusty-CI` supports writing a Makefile, or writing a shell script to install dependencies. More buildsystems should be added in the future.

To output a Makefile for installation, run one of the following commands.

```bash
# Output an install makefile
rusty-ci --make install

# Identical
rusty-ci -m install

# Run the makefile to install
make
```

To output a bash script for installation, run one of the following commands.

```bash
# Output an install makefile
rusty-ci --bash install

# Identical
rusty-ci -b install

# Make the shell script executable
chmod +x ./install.sh
# Run the install script
./install.sh
```

If you do decide to either the bash or make buildsystems, be sure to read `rusty-ci`'s output and follow any instructions given.

## After installing

For some reason, when `pip` and `apt` fail to install an application, they don't seem to return a non-zero exit code. This makes it difficult to capture errors and debug your installation problems. 

Even if Rusty-CI tells you it has succeeded installing its dependencies, be sure to verify this using the following commands.

```bash
# Each of these should output a help message
python3 -m venv
buildbot
buildbot-worker
```

If any of these do not display a help message, rerun the `install` subcommand and _**read every line of output from rusty-ci**_. There might be an instruction or a warning that you missed!

## The Build Subcommand

The `build` subcommand is responsible for constructing the buildbot master, the buildbot workers, and their respective configuration files from a YAML file.

The build subcommand can also be used with different buildsystems. However, every system for building currently uses the same code for the build `subcommand`
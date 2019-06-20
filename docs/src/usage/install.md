# Install Subcommand

The `install` subcommand is responsible for installing Rusty-CI's dependencies. When you run the install subcommand, it will output a bash script that uses your existing [python3](https://www.python.org/) installation to install [buildbot](https://buildbot.net/) and its dependencies.

You can also choose to output a makefile instead of a bash script, but it's not really necessary.

## Usage

To output a Makefile for installation, run one of the following commands.

```bash
# Output an install makefile
rusty-ci install --make

# Identical
rusty-ci install -m

# Run the makefile to install
make
```

To output a bash script for installation, run one of the following commands.

```bash
# Output an install shell script
rusty-ci install

# Identical
rusty-ci install --bash

# Make the shell script executable
chmod +x ./install.sh
# Run the install script
./install.sh
```

If you do decide to either the bash or make buildsystems, be sure to read `rusty-ci`'s output and follow any instructions given.

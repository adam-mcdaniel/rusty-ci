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


```
$ rusty-ci install

==[INFO]===> Installing dependencies for rusty-ci...
Do you already have python3-dev, python3-pip, and python3-venv installed? (y/n) y
==[INFO]===> Writing install file to `./install.sh`
==[INFO]===> Successfully wrote install file
==[WARN]===> To install dependencies run `install.sh`
==[WARN]===> Before building from a YAML file, be sure to run `. venv/bin/activate`
==[INFO]===> Next, write your VCS's api token to 'auth.token', and then run the `build` subcommand
Successfully finished install
```

Now, get an access token from your version control system, and write it to a file named `auth.token`. Next, run the move on to the `build` subcommand.
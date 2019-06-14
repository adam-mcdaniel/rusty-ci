# Install Subcommand

The `install` subcommand is responsible for installing Rusty-CI's dependencies. When you run the install subcommand, it will run several shell commands in the background to install [python3](https://www.python.org/), and [buildbot](https://buildbot.net/).

That is, if you use the default buildsystem. If you do not want Rusty-CI running anything on your machine without supervision, you can choose from the other buildsystems. Right now, `Rusty-CI` supports writing a Makefile, or writing a shell script to install dependencies. More buildsystems should be added in the future.

## Usage

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

#### After installing

For some reason, when `pip` and `apt` fail to install an application, they don't seem to return a non-zero exit code. This makes it difficult to capture errors and debug your installation problems. 

Even if Rusty-CI tells you it has succeeded installing its dependencies, be sure to verify this using the following commands.

```bash
# Each of these should output a help message
python3 -m venv
buildbot
buildbot-worker
```

If any of these do not display a help message, rerun the `install` subcommand and _**read every line of output from rusty-ci**_. There might be an instruction or a warning that you missed!

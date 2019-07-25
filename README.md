# rusty-ci

A tool to generate buildbot projects from a YAML file


## Documentation

You can find the usage documentation [here](https://adam-mcdaniel.github.io/rusty-ci/), and the code documentation [here](https://docs.rs/rusty-ci).

## Installation

Install rust.

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install `rusty-ci`

```bash
cargo install rusty-ci
```

Install python3, pip, and venv

```
sudo apt update
sudo apt upgrade
sudo apt install python3-dev python3-pip python3-venv
```

## Usage

To start, run `rusty-ci setup`, and carefully read and follow the output's instructions.


## Recommendations

I highly recommend using this in a linux-container to avoid poisoning your OS's environment. If you do decide to use a linux-container, be sure to `apt update && apt upgrade`, and `apt install build-essential` before doing anything though!


# TL;DR

Just paste this stuff into your terminal to install and setup (I'm assuming you're using a Debian based OS).


```bash
# Update && Upgrade
apt update && apt upgrade
apt install build-essential python3-dev python3-pip python3-venv

# Install rust
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo install -f rusty-ci


# Write template yaml files
rusty-ci setup

# Assuming that you told rusty-ci to output to template.yaml
# Edit your file as needed
nano template.yaml

# Assuming that you told rusty-ci to output to mail.yaml
# Edit your file as needed
nano mail.yaml


# Install rusty-ci dependencies
rusty-ci install
chmod +x ./install.sh
./install.sh

# Enter venv
. venv/bin/activate

# Add an authentication token from your VCS (github)
nano auth.token

rusty-ci build template.yaml --mail mail.yaml
# All done!
```
# rusty-ci

A tool to generate [buildbot](https://buildbot.net/) projects from a YAML file


## Description

Rusty-CI is meant to be a simple continuous integration tool that takes very little time to set up. Within 10 minutes of reading this README, you could have Rusty-CI testing your repository!

It works by constructing a webserver and several workers from one or two YAML files that describe how you want your project to be tested. When Rusty-CI detects a change in your repository, it will use the data from your YAML files to determine how you want that branch to be tested. Then, it will push a status report to your VCS.

## Features

- Webgui for detailed test output and CI configuration
- Detecting merge / pull requests on GitHub and GitLab
- Use regular expressions to trigger tests for specific file changes on specific branches
- Tests that can depend on one another
- Custom testing scripts
- Customizable email bot settings

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
apt update -y && apt upgrade -y
apt install -y build-essential python3-dev python3-pip python3-venv

# Install rust
curl https://sh.rustup.rs -sSf | sh # Run the rust installer
source $HOME/.cargo/env             # Add `cargo` to your path
cargo install -f rusty-ci           # Install the latest rusty-ci release


# Write template yaml files
rusty-ci setup template.yaml mail.yaml

# Uncomment to modify your CI's settings to fit your project
# nano template.yaml # Controls how your CI tests your code
# nano mail.yaml     # Defines email update / notification settings

# Install rusty-ci dependencies
rusty-ci install -q   # Build install.sh
chmod +x ./install.sh # Make install.sh executable
./install.sh          # Install!

# Enter venv
. venv/bin/activate   # Enter the venv created by rusty-ci
                      # to avoid poisoning your environment

# Add an authentication token from your VCS (github)
echo "YOUR AUTH TOKEN HERE" > auth.token

# Construct your ci bot
rusty-ci build -q template.yaml --mail mail.yaml

# Spin up the workers!
rusty-ci start template.yaml -q

# All done!
```


## License
Rusty-CI is distributed under the terms of the Apache License (Version 2.0).

See LICENSE for details.
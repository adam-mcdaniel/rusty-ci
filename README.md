# rusty-ci

A tool to generate buildbot projects from a YAML file


## Documentation

You can find the documentation [here](https://adam-mcdaniel.github.io/rusty-ci/).

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
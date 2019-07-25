# TLDR

If you were too bored to read the whole thing, just paste this stuff into your terminal (I'm assuming you're using a Debian based OS).


```bash
# Update && Upgrade
apt update && apt upgrade
apt install build-essential python3-dev python3-pip python3-venv

# Install rust
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo install -f rusty-ci

# Install rusty-ci dependencies
rusty-ci install
chmod +x ./install.sh
./install.sh

# Enter venv
. venv/bin/activate

rusty-ci setup
# Assuming that you told rusty-ci to output to template.yaml
# Edit your file as needed
nano template.yaml

# Assuming that you told rusty-ci to output to mail.yaml
# Edit your file as needed
nano mail.yaml

rusty-ci build template.yaml --mail mail.yaml
# All done!
```
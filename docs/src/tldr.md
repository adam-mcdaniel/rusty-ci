# TLDR

If you were too bored to read the whole thing, just paste this stuff into your terminal (I'm assuming you're using a Debian based OS).


```bash
# Update && Upgrade
sudo apt update
sudo apt upgrade
sudo apt install build-essential

# Install rust
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo install rusty-ci -f

# Install rusty-ci dependencies
rusty-ci -b install
chmod +x ./install.sh
sudo ./install.sh

# Enter venv
. venv/bin/activate

rusty-ci setup
# Assuming that you told rusty-ci to output to template.yaml
# Edit your file as needed
nano template.yaml

rusty-ci build template.yaml
# All done!
```
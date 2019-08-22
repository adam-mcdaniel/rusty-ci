# TL;DR

If you were too bored to read the whole thing, just paste this stuff into your terminal (I'm assuming you're using a Debian based OS).


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
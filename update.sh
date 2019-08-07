#! /bin/bash
# Run in linux container running rusty-ci!
# To use this, run rusty-ci install first.

. ~/.bashrc
. ~/.cargo/env

cd ~
git clone https://github.com/adam-mcdaniel/rusty-ci
cd rusty-ci
git reset --hard
git pull origin master


cargo install -f --path .

. venv/bin/activate

# Kill rusty-ci before it kills us!
rusty-ci build -q rusty_ci.yaml
rusty-ci start -q rusty_ci.yaml
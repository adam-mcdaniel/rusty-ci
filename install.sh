#!/bin/sh

sudo apt-get install python3-dev -y
sudo apt-get install python3-pip -y
sudo apt-get install python3-venv -y

python3 -m venv venv
. venv/bin/activate; python3 -m pip install -U pip; python3 -m pip install 'buildbot[bundle]';
. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial


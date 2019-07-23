#!/bin/sh

python3 -m venv venv
. venv/bin/activate; python3 -m pip install -U pip; python3 -m pip install txrequests treq 'buildbot[bundle]';
. venv/bin/activate; python3 -m pip install buildbot-worker setuptools-trial


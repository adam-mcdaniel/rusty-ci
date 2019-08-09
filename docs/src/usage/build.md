# Build Subcommand

The `build` subcommand is responsible for constructing the buildbot master, the buildbot workers, and their respective configuration files from a YAML file.

```
rusty-ci-build x.x.x
Adam McDaniel <adam.mcdaniel17@gmail.com>
Build rusty-ci from YAML file(s)

USAGE:
    rusty-ci build [FLAGS] [OPTIONS] <MASTER_YAML>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Don't ask user anything
    -V, --version    Prints version information

OPTIONS:
    -m, --mail <MAIL_YAML>    The path to the YAML file dedicated to SMTP authentication info for sending email
                              notifications

ARGS:
    <MASTER_YAML>    The path to the master YAML file
```

## Usage

<!-- Before you build your YAML file, you need to get an API or authentication token from your respective Version Control System, and write it to `auth.token`. This is so the output buildbot project has access to push commit statuses to your repository, and other things like that. -->

First, confirm that you're inside your python virtual environment.

```bash
. venv/bin/activate
```

To build from a YAML file, simply run this command.

```bash
rusty-ci build template.yaml
```

If you want to build your CI with support for email notifications, run it like so.

```bash
rusty-ci build template.yaml --mail mail.yaml
# is identical to the following
rusty-ci build template.yaml -m mail.yaml
```

Now, run the start subcommand.

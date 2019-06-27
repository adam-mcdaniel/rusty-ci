# Build Subcommand

The `build` subcommand is responsible for constructing the buildbot master, the buildbot workers, and their respective configuration files from a YAML file.

```
rusty-ci-build 0.1.0
Adam McDaniel <adam.mcdaniel17@gmail.com>
Build rusty-ci from an input yaml file

USAGE:
    rusty-ci build [OPTIONS] <MASTER_YAML>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mail <MAIL_YAML>    The path to the YAML file dedicated to SMTP authentication info for sending email
                              notifications

ARGS:
    <MASTER_YAML>    The path to the YAML file
```

## Usage

Before you build your yaml file, you need to get an API or authentication token from your respective Version Control System, and write it to `auth.token`. This is so the output buildbot project has access to push commit statuses to your repository, and other things like that.

To build from a yaml file, simply run this command.

```bash
rusty-ci build my_ci.yaml
```

If you want to build your CI with support for email notifications, run it like so.

```bash
rusty-ci build my_ci.yaml --mail my_mail.yaml
# identical to the following
rusty-ci build my_ci.yaml -m my_mail.yaml
```

Now, run the start subcommand.

# Build Subcommand

The `build` subcommand is responsible for constructing the buildbot master, the buildbot workers, and their respective configuration files from a YAML file.

```
rusty-ci-build 0.1.0
Adam McDaniel <adam.mcdaniel17@gmail.com>
Build rusty-ci from an input yaml file

USAGE:
    rusty-ci build <YAML>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <YAML>    The path to the YAML file
```

## Usage

Before you build your yaml file, you need to get an API or authentication token from your respective Version Control System, and write it to `auth.token`. This is so the output buildbot project has access to push commit statuses to your repository, and other things like that.

To build from a yaml file, simply run this command.

```bash
rusty-ci build my_ci.yaml
```

Now, run the start subcommand.

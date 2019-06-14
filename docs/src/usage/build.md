# Build Subcommand

The `build` subcommand is responsible for constructing the buildbot master, the buildbot workers, and their respective configuration files from a YAML file.

The build subcommand can also be used with different buildsystems. However, every system for building currently uses the same underlying code for building, so there's really no benefit to using a particular build system for now.


## Usage

To build from a yaml file, simply run this command.

```bash
rusty-ci build my_ci.yaml
```

Now, to view your web gui, go to [http://localhost:8010](http://localhost:8010).

## Not working?

If your web gui isn't loading, buildbot probably failed to start the master, or you put the wrong IP in the master section of your yaml file.

You probably just forgot to change the path to one of your worker's working directories, though.

To see the log for the master, run `tail master/twistd.log`.

If you see an exception, then the master ran into an error. This error will most likely be self explanatory and easy to debug, but in the case that it isn't, go to the [buildbot website](https://buildbot.net).
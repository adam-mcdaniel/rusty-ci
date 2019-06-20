# Start Subcommand

# Usage

Before doing anything, confirm you're in your venv by running `. venv/bin/activate`!

After running all the other rusty-ci subcommands, run `rusty-ci start YOUR_YAML_FILE.yaml`

This will kill the master and workers that were previously running, and start new instances of them.

Now, to view your web gui, go to [http://localhost:8010](http://localhost:8010).

## Not working?

If your web gui isn't loading, buildbot probably failed to start the master, or you put the wrong IP in the master section of your yaml file.

You probably just forgot to change the path to one of your worker's working directories, though.

To see the log for the master, run `tail -f master/twistd.log`.

If you see an exception, then the master ran into an error. This error will most likely be self explanatory and easy to debug, but in the case that it isn't, go to the [buildbot website](https://buildbot.net).

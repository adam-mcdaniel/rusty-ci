# Start Subcommand

## Usage

After running all the other rusty-ci subcommands, run

```bash
rusty-ci start template.yaml
```

This will kill the master and workers that were previously running, and start new instances of them.

Now, to view your web gui, go to [http://localhost:8010](http://localhost:8010).


If you want to RESTART your CI without killing it, use the following subcommand instead.

```bash
rusty-ci rebuild template.yaml mail.yaml
```

This will not start your CI if it hasn't already been started!

## Not working?

Confirm you're in your python virtual environment!

If your web gui isn't loading, buildbot probably failed to start the master, or you put the wrong IP in the master section of your YAML file.

You probably just forgot to change the path to one of your worker's working directories, though.

To see the log for the master, run `tail -f master/twistd.log`.

If you see an exception, then the master ran into an error. This error will most likely be self explanatory and easy to debug, but in the case that it isn't, go to the [buildbot website](https://buildbot.net).

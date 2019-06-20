# Setup Subcommand

The setup command is very simple. All it does is write a template YAML file to any path you'd like.

```
$ ./rusty-ci setup
Where do you want the output template yaml to be? out.yaml
Are you sure? (y/n) y
==[INFO]===> Writing template yaml file to out.yaml...
==[INFO]===> All done!
```

```
$ more out.yaml

# This section holds data specific to the master of the workers
master:
  # The title subsection of the master holds the title of your web gui
  title: "Rusty-CI"
  title-url: "https://github.com/adam-mcdaniel/rusty-ci"

  # This is the ip of the web-gui
  # The port is 8010
  webserver-ip: localhost

  # The address of your repository
  repo: "https://github.com/adam-mcdaniel/rusty-ci"

  # The number of seconds to wait before checking for updates on your repository
  # Two minutes is a good poll interval
  poll-interval: 120

# This section holds data specific to the handler that will look for
# pull requests / merge requests on your repository
merge-request-handler:
  # This is basically the website you're using for version control
  # Right now, github is the only supported site
...
```

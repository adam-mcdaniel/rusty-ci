# Setup Subcommand

The setup command is very simple. All it does is write template YAML files for building your CI and controlling email notifications to any paths you'd like.

```
$ ./rusty-ci setup
Where do you want the template master yaml file to be? out.yaml
Are you sure? (y/n) y
==[INFO]===> Writing template yaml file to out.yaml...
Where do you want the template mail yaml file to be? mail.yaml
Are you sure? (y/n) y
==[INFO]===> Writing template yaml file to mail.yaml...
==[INFO]===> All done!
==[INFO]===> Next, run the `install` subcommand command using either the `bash` or `make` flag
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

  # The number of seconds to wait before checking for updates on your
repository
  # Two minutes is a good poll interval
  poll-interval: 120

# This section holds data specific to the handler that will look for
# pull requests / merge requests on your repository
merge-request-handler:
  # This is basically the website you're using for version control
  # Right now, github is the only supported site
...
```

```
$ more mail.yaml

# Rusty-CI will automatically email "interested users" about
# all tests that run. The list of "interested users" is the
# list of people who have a commit in the branch or pull request.

# The extra recipients to email
extra-recipients:
  # Emails under the failure section will be emailed
  # info about every failed build
  failure:
    - failure@gmail.com
  # Emails under the success section will be emailed
  # info about every successful build
  success:
    - success@gmail.com
  # Emails under the all section will be emailed
  # info about every build
  all:
    - all_tests@gmail.com


# The "from" email address used to send email updates to recipients
from-address: your-email-here@gmail.com

# The suffix to add to the interested users' usernames
# to get an email we can send updates to.
lookup: gmail.com

# The smtp relay hostname (self explanatory)
# gmail's smtp relay hostname is `smtp.gmail.com`
smtp-relay-host: smtp.gmail.com

# The smtp relay port (self explanatory)
# 587 is the smtp port that `smtp.gmail.com` uses
smtp-port: 587

# The password used to login to the "from" email address account
smtp-password: "p@$$w0rd"
```

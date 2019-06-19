# Introduction

## What is Rusty-CI?

Rusty-CI is just a user interface for [buildbot](https://buildbot.net). Instead of having to write the Python for your CI project to get the versatility you want, just fill out Rusty-CI's template YAML file and it'll do the rest for you.

## Usage

Here is the template YAML file that Rusty-CI will output for you to fill out with your own data.

It explains itself for the most part, but I'll be covering it in more detail in a later chapter.

```yaml
# This section holds data specific to the master of the workers
master:
  # The title subsection of the master holds the title of your web gui
  title:
    - "Rusty-CI"
  title-url:
    - "https://github.com/adam-mcdaniel/rusty-ci"

  # This is the ip of the web-gui
  # The port is 8010
  webserver-ip:
    - localhost

  # The address of your repository
  repo:
    - "https://github.com/adam-mcdaniel/rusty-ci"

  # The number of seconds to wait before checking for updates on your repository
  poll-interval:
    - 120

# This section holds data specific to the handler that will look for 
# pull requests / merge requests on your repository
merge-request-handler:
  # This is basically the website you're using for version control
  # Right now, github is the only supported site
  # If you're using an unsupported version control system, no worries,
  # rusty-ci just wont run on pull requests.
  version-control-system:
    - github
  # The username of the owner of the repository
  owner:
    - adam-mcdaniel

  # The name of the repository
  repo-name:
    - rusty-ci

  # You dont want to run arbitrary code on your machine when anyone
  # makes a pull request. Rusty-CI will not test anyone's pull request
  # if their username is not in this list.
  whitelist:
    - adam-mcdaniel

  # This is the authentication token from your version control system
  auth-token:
    - "just-testing"


# This section holds each worker
# You can have as many workers as youd like, just be sure to fill out
# each of their fields out properly.
workers:
  # The name of this worker is `test-worker`
  test-worker:
    # The ip of the master
    masterhost:
      - localhost
    # The port of the master
    # This is not the same as the web gui port!
    masterport:
      - 9989
    # The absolute path to the working directory of this worker
    basedir:
      - '/home/adam/Desktop/rusty-ci/testing/test-worker'
    # The password for this worker
    # This is used by the master to give the worker a job
    password:
      - pass

# This section holds each scheduler.
# Like the workers section, you may have as many schedulers as youd like.
schedulers:
  # Create a scheduler named `ci-change`
  # This scheduler will trigger the `rusty-ci-test` builder whenever it
  # detects a change in a yaml file for any branch.
  ci-change:
    # This is a regular expression that matches a branch.
    # If there is a change in a branch whos name matches this regex,
    # it will be checked by the following triggers section.
    # THIS WILL ONLY USE THE FIRST REGULAR EXPRESSION IN THIS SECTION TO MATCH THE BRANCH
    branch:
      - ".*"
    # If a change has occurred in a branch that matches the regex in the branch section,
    # Then the files that were changed are matched against the regular expressions in the
    # triggers section. You can have any number of regular expressions in the triggers section.
    # If any one of them matches the name of a file that was changed in a matched branch,
    # then the builders in this scheduler's `builders` section are executed.
    triggers:
      - '.*\.yaml'
      - '.*\.sh'
      - '.*Makefile'
    # This scheduler triggers the `rusty-ci-test` builder.
    # You can put as many builders as youd like here, and the scheduler will start them all.
    builders:
      - rusty-ci-test

# These are the builders that are executed by the schedulers
# Each has its own specific task that is delegated to one or more workers
# When a builder is run, its script is run on the command line.
# You can have as many builders as youd like as well.
builders:
  # The name of the builder is `rusty-ci-test`
  rusty-ci-test:
    # This is the shell script that the workers will run when this builder is executed
    # You can have as many instructions as youd like
    # Mind you, you cannot use the |, >, <, >>, <<... operators. Sadly, buildbot
    # passes each item separated by whitespace as another parameter to function.
    script:
      - echo Hello world!
      - echo Im an instruction in a script!
    # These are the workers to delegate this build job to
    workers:
      - test-worker
    # The repo to refresh from before running
    repo:
      - "https://github.com/adam-mcdaniel/rusty-ci"
```
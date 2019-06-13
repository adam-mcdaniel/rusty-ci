
use rusty_yaml::Yaml;

use std::fmt::{Display, Error, Formatter};
use std::process::exit;

/// A version control system is a system that allows programmers to manage
/// changes on a product in development. A few examples include, but are not limited to,
/// `GitHub`, `GitLab`, `Mercurial`.
pub enum VersionControlSystem {
    GitHub,
    Unsupported,
}

/// The purpose of a continuous integration tool is to continuously confirm the
/// validity and robustness of code. It follows then that you must check code BEFORE
/// it is deployed. To do this, you must take the code that someone wants to merge into
/// the repository, and test what the merged code would look like. This struct
/// allows us to add this functionality to the output buildbot project.
pub struct MergeRequestHandler {
    /// A VCS is a verison control system. This is used to determine
    /// how to tailor the output Python code to the specific VCS.
    /// In the future, we should implement the abstractions for the
    /// VCS in the Python, instead of abstracting it in the Rust.
    /// The VCS, currently, must be one of:
    /// - github
    vcs: VersionControlSystem,
    /// The username of the owner of the repository
    owner: String,
    /// The name of the repo
    /// The name of the rusty-ci repo, for example,
    /// is just `rusty-ci`, not the entire url.
    repo_name: String,
    /// Running code from pull requests is dangerous: the request could contain
    /// malicious code. To stop anyone from executing arbitrary code on our machines,
    /// we must have a whitelist. This list contains the usernames of people that the CI
    /// will run code for.
    /// If the whitelist contains "adam-mcdaniel", and a user named "adam-mcdaniel" makes
    /// a pull request on my repository, rusty-ci will run the code in his PR. If his username
    /// is `im_not_in_the_whitelist` then his code will not be run on our machines until his
    /// username is added to the whitelist.
    whitelist: Vec<String>,
    /// This is the authentication token for the VCS
    auth_token: String,

    /// This field is not to be changed by the user because if youre using something other
    /// than git, youre doing it wrong.
    repository_type: String,
}


impl MergeRequestHandler {
    pub fn new(
        vcs: VersionControlSystem,
        owner: String,
        repo_name: String,
        whitelist: Vec<String>,
        auth_token: String,
    ) -> Self {
        Self {
            vcs,
            owner,
            repo_name,
            whitelist,
            auth_token,
            repository_type: String::from("git"), // We dont support any other repo type.
        }
    }
}

/// This trait implementation tells rust how to convert a MergeRequestHandler object
/// into the output python.
impl Display for MergeRequestHandler {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self.vcs {
            VersionControlSystem::GitHub => write!(
                f,
                "whitelist_authors = {:?}

def github_pull_check(pull_request):
    sender = pull_request[\"user\"][\"login\"]
    for author in whitelist_authors:
        if author == sender:
            return True
    return False

c['change_source'].append(changes.GitHubPullrequestPoller(
        owner=\"{owner}\",
        repo=\"{name}\",
        # right now just poll every 10 seconds
        # this will need to change in the future, but this is just for testing.
        pollInterval=10,
        pullrequest_filter=github_pull_check,
        repository_type=\"{repository_type}\",
        token=\"{token}\"))

",
                self.whitelist,
                token = self.auth_token.trim_matches('"'),
                name = self.repo_name.trim_matches('"'),
                owner = self.owner.trim_matches('"'),
                repository_type = self.repository_type.trim_matches('"'),

            ),
            VersionControlSystem::Unsupported => write!(
                f,
                "print('We currently dont support building merge requests on your VCS.')"
            ),
        }
        // def github_pull_check(pull_request):
        //     sender = pull_request['user']["login"]
        //     for author in whitelist_authors:
        //         if author == sender:
        //             return True
        //     return False
        //
        // c['change_source'].append(changes.GitHubPullrequestPoller(
        //         owner="adam-mcdaniel",
        //         repo="rusty-ci",
        //         pollInterval=10, pullrequest_filter=github_pull_check, repository_type="git",
        //         token="645e23fc7a41d0afb98bd8aa55c1edd345795315"))

    }
}


impl From<Yaml> for MergeRequestHandler {
    fn from(yaml: Yaml) -> Self {
        // Confirm that the merge request handler has the required sections
        for section in [
            "version-control-system",
            "owner",
            "repo-name",
            "whitelist",
            "auth-token",
        ]
        .iter()
        {
            if !yaml.has_section(section) {
                error!("There was an error creating the merge request handler: '{}' section not specified", section);
                exit(1);
            }
        }
        // Now that we've verified the required sections exist, continue


        let vcs: VersionControlSystem = match yaml
            .get_section("version-control-system")
            .unwrap()
            .nth(0)
            .unwrap()
            .to_string()
            .as_str()
        {
            "github" => VersionControlSystem::GitHub,
            _ => {
                warn!(
                    "We do not support building merge requests on your version control system yet!"
                );
                warn!(
                    "We will proceed with the build. All other features should function as intended."
                );
                VersionControlSystem::Unsupported
            }
        };

        // Get the username of the owner of the repository
        let owner: String = yaml
            .get_section("owner")
            .unwrap()
            .nth(0)
            .unwrap()
            .to_string();

        // Get the name of the repository
        let repo_name: String = yaml
            .get_section("repo-name")
            .unwrap()
            .nth(0)
            .unwrap()
            .to_string();

        // Iterate over the whitelist section to get the names
        // of the whitelisted authors
        let mut whitelist: Vec<String> = vec![];
        for author in yaml.get_section("whitelist").unwrap() {
            whitelist.push(author.to_string());
        }

        // Get the authentication token
        let auth_token: String = yaml
            .get_section("auth-token")
            .unwrap()
            .nth(0)
            .unwrap()
            .to_string();

        // Return the constructed Self
        Self::new(vcs, owner, repo_name, whitelist, auth_token)
    }
}
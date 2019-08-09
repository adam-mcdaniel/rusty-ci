use crate::{unwrap, File};

use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;

/// A version control system is a system that allows programmers to manage
/// changes on a product in development. A few examples include, but are not limited to,
/// `GitHub`, `GitLab`, `Mercurial`.
pub enum VersionControlSystem {
    GitHub,
    GitLab,
    Unsupported,
}

/// This is the path to the file containing the auth / api token
/// for the version control system
pub const AUTH_TOKEN_PATH: &str = "auth.token";

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
    /// username is added to the whitelist, or a whitelisted user grants permission to test.
    whitelist: Vec<String>,
    /// This is the authentication token for the VCS for write access to the repository
    auth_token: String,
    /// This field is not to be changed by the user because if youre using something other
    /// than git, youre doing it wrong :)
    repository_type: String,
}

impl MergeRequestHandler {
    pub fn new(
        vcs: VersionControlSystem,
        owner: String,
        repo_name: String,
        whitelist: Vec<String>,
    ) -> Self {
        let auth_token = match File::read(AUTH_TOKEN_PATH) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                error!(
                    "Could not read authentication token from file '{}' because {}",
                    AUTH_TOKEN_PATH, e
                );
                exit(1);
            }
        };

        if auth_token.len() == 0 {
            error!(
                "You didn't write your VCS's authentication token to '{}'!",
                AUTH_TOKEN_PATH
            );
            exit(0);
        }

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

try:
    c['change_source'].append(changes.GitHubPullrequestPoller(
            owner=\"{owner}\",
            repo=\"{name}\",
            # right now just poll every 60 seconds
            # this will need to change in the future, but this is just for testing.
            pollInterval=120,
            repository_type=\"{repository_type}\",
            github_property_whitelist=[\"*\"],
            token=\"{token}\"))
except Exception as e:
    print(f\"Could not create merge request handler: {{str(e)}}\")


context = util.Interpolate(\"%(prop:buildername)s\")
github_status_service = reporters.GitHubStatusPush(token='{token}',
                                context=context,
                                startDescription='Build started.',
                                endDescription='Build done.')

c['services'].append(github_status_service)


def is_whitelisted(props, password):
    for prop in ['github.number', 'github.comments_url', 'github.user.login']:
        # If these properties arent present, its not a pull request
        if not (props.hasProperty(prop)):
            return True
    
    # URL for comments info
    comments_url = props['github.comments_url']

    # The pull request number that we'll try to whitelist
    pr_number = props['github.number']

    # The author of the PR
    author = props['github.user.login']

    resp = req.get(comments_url)
    try:
        # Try to convert to a JSON object so we can read the data
        json_acceptable_string = resp.text.replace(\"'\", \"\\\"\")
        comments_json = json.loads(json_acceptable_string)


        # Check each comment
        for comment in comments_json:
            # If the comment was made by an admin and matches the password
            if comment['user']['login'] in whitelist_authors and re.fullmatch(password, comment['body']):
                # If the pull request was not already in the whitelisted PRs, add it
                print(\"ADMIN: \" + str(comment['user']['login']) + \" PASSWORD: \" + str(comment['body']))
                print(f\"PR NUMBER {{pr_number}} IS GOOD TO TEST\")
                return True
    except Exception as e:
        # There was a problem converting to JSON, github returned bad data
        print(f\"There was an error: {{str(e)}}. If this error has anything to do with JSON, its likely that you've queried GitHub too many times.\")
        # Write the returned webpage to BAD
        open('BAD', 'w').write(resp.text)

    
    if author in whitelist_authors:
        print(\"WHITELISTED AUTHOR\")
        return True

    return False
",
                self.whitelist,
                token = self.auth_token.trim_matches('"'),
                name = self.repo_name.trim_matches('"'),
                owner = self.owner.trim_matches('"'),
                repository_type = self.repository_type.trim_matches('"'),

            ),
            VersionControlSystem::GitLab => write!(
                f,
                "
def is_whitelisted(props, password): return True

context = util.Interpolate(\"%(prop:buildername)s\")
gitlab_status_service = reporters.GitLabStatusPush(token='{token}',
                                context=context,
                                startDescription='Build started.',
                                endDescription='Build done.')

c['services'].append(gitlab_status_service)
               
",
                token = self.auth_token.trim_matches('"'),
            ),
            VersionControlSystem::Unsupported => write!(
                f,
                "print('We currently dont support building merge requests on your VCS.')"
            ),
        }
    }
}

impl From<Yaml> for MergeRequestHandler {
    fn from(yaml: Yaml) -> Self {
        // Confirm that the merge request handler has the required sections
        for section in ["version-control-system", "owner", "repo-name", "whitelist"].iter() {
            if !yaml.has_section(section) {
                error!("There was an error creating the merge request handler: '{}' section not specified", section);
                exit(1);
            }
        }
        // Now that we've verified the required sections exist, continue

        let vcs: VersionControlSystem = match unwrap(&yaml, "version-control-system").as_str() {
            "github" => VersionControlSystem::GitHub,
            "gitlab" => VersionControlSystem::GitLab,
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
        let owner: String = unwrap(&yaml, "owner");

        // Get the name of the repository
        let repo_name: String = unwrap(&yaml, "repo-name");

        // Iterate over the whitelist section to get the names
        // of the whitelisted authors
        let mut whitelist: Vec<String> = vec![];
        for author in yaml.get_section("whitelist").unwrap() {
            whitelist.push(
                author
                    .to_string()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
        }

        // Return the constructed Self
        Self::new(vcs, owner, repo_name, whitelist)
    }
}

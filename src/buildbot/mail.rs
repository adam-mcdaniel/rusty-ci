
use crate::{unmatched_quotes, unwrap};
use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;


/// This object is responsible for building the `MailNotifier` object
/// in the buildbot master config. It contains the information for
/// authenticating an SMTP request to send email. This information is
/// sensitive and should be kept separate from the master yaml file
pub struct MailNotifier {
    /// These are the recipients that will be messages with every test
    all_recipients: Vec<String>,
    /// These are the recipients that will be messages with every successful test
    success_recipients: Vec<String>,
    /// These are the recipients that will be messages with every failed test
    failure_recipients: Vec<String>,

    /// The address that the emails will be sent from
    from_address: String,
    /// The smtp host that will be used to access the email
    smtp_relay_host: String,
    /// The port of the smtp_relay_host to use
    smtp_port: String,

    /// Identical to from_address
    smtp_user: String,

    /// Basically this is the suffix to the email address
    /// to tack on to the end of the usernames of the interested users.
    /// So, the user `dn-lang` will be emailed at `dn-lang@example.org`
    /// if lookup is `example.org`
    lookup: String,

    /// The password to access the from email address
    smtp_password: String,
}

impl MailNotifier {
    pub fn new(
        all_recipients: Vec<String>,
        success_recipients: Vec<String>,
        failure_recipients: Vec<String>,
        from_address: String,
        smtp_relay_host: String,
        smtp_port: String,
        lookup: String,
        smtp_password: String,
    ) -> Self {
        Self {
            all_recipients,
            success_recipients,
            failure_recipients,
            from_address: from_address.clone(),
            smtp_relay_host,
            smtp_port,
            smtp_user: from_address,
            lookup,
            smtp_password,
        }
    }
}


impl Display for MailNotifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            r#"

# The mail notifier responsible for all info
all = reporters.MailNotifier(fromaddr="{from_address}",
                            sendToInterestedUsers=True,
                            extraRecipients={all_recipients},
                            lookup="{lookup}",
                            relayhost="{relay_host}", smtpPort={port},
                            smtpUser="{user}", buildSetSummary=True,
                            # addLogs=True,
                            mode="all",
                            smtpPassword="{password}")
c['services'].append(all)


# The mail notifier responsible for failures
failures = reporters.MailNotifier(fromaddr="{from_address}",
                            sendToInterestedUsers=True,
                            extraRecipients={failure_recipients},
                            lookup="{lookup}",
                            relayhost="{relay_host}", smtpPort={port},
                            smtpUser="{user}", buildSetSummary=True,
                            # addLogs=True,
                            mode="failing",
                            smtpPassword="{password}")
c['services'].append(failures)



# The mail notifier responsible for successes
successes = reporters.MailNotifier(fromaddr="{from_address}",
                            sendToInterestedUsers=True,
                            extraRecipients={success_recipients},
                            lookup="{lookup}",
                            relayhost="{relay_host}", smtpPort={port},
                            smtpUser="{user}", buildSetSummary=True,
                            # addLogs=True,
                            mode="passing",
                            smtpPassword="{password}")
c['services'].append(successes)


"#,
            all_recipients = format!("{:?}", self.all_recipients),
            success_recipients = format!("{:?}", self.success_recipients),
            failure_recipients = format!("{:?}", self.failure_recipients),
            from_address = self.from_address,
            relay_host = self.smtp_relay_host,
            password = self.smtp_password,
            user = self.smtp_user,
            port = self.smtp_port,
            lookup = self.lookup,
        )
    }
}


impl From<Yaml> for MailNotifier {
    fn from(yaml: Yaml) -> Self {
        
        // Verify that the yaml file doesnt have unmatched quotes!
        match unmatched_quotes(&yaml) {
            Some(line) => {
                error!("There was a problem creating the mail notifier: unmatched quotes in the line '{}'", line.trim());
                exit(1);
            },
            _ => {}
        }
        
        // Confirm that the merge request handler has the required sections
        for section in [
            "extra-recipients",
            "from-address",
            "smtp-relay-host",
            "smtp-port",
            "lookup",
            "smtp-password",
        ]
        .iter()
        {
            if !yaml.has_section(section) {
                error!(
                    "There was a problem creating the mail notifier: '{}' section not specified.",
                    section
                );
                exit(1);
            }
        }


        let extra_recipients = yaml.get_section("extra-recipients").unwrap();

        for section in ["all", "failure", "success"].iter() {
            if !extra_recipients.has_section(section) {
                error!("There was a problem creating the mail notifier: '{}' section not specified in the 'extra-recipients' subsection.", section);
                exit(1);
            }
        }
        let mut all_recipients = vec![];
        for recipient in extra_recipients.get_section("all").unwrap() {
            all_recipients.push(recipient.to_string());
        }

        let mut success_recipients = vec![];
        for recipient in extra_recipients.get_section("success").unwrap() {
            success_recipients.push(recipient.to_string());
        }

        let mut failure_recipients = vec![];
        for recipient in extra_recipients.get_section("failure").unwrap() {
            failure_recipients.push(recipient.to_string());
        }


        let from_address = unwrap(&yaml, "from-address");
        let smtp_relay_host = unwrap(&yaml, "smtp-relay-host");
        let smtp_port = unwrap(&yaml, "smtp-port");
        let smtp_password = unwrap(&yaml, "smtp-password");
        let lookup = unwrap(&yaml, "lookup");

        Self::new(
            all_recipients,
            success_recipients,
            failure_recipients,
            from_address,
            smtp_relay_host,
            smtp_port,
            lookup,
            smtp_password,
        )
    }
}

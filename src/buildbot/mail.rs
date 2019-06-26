
use crate::unwrap;
use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;


/// This object is responsible for building the `MailNotifier` object
/// in the buildbot master config. It contains the information for
/// authenticating an SMTP request to send email. This information is
/// sensitive and should be kept separate from the master yaml file
pub struct MailNotifier {
    /// These are the recipients that will be messages with every test
    extra_recipients: Vec<String>,
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
        extra_recipients: Vec<String>,
        from_address: String,
        smtp_relay_host: String,
        smtp_port: String,
        lookup: String,
        smtp_password: String,
    ) -> Self {
        Self {
            extra_recipients,
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

mail_notifier_service = reporters.MailNotifier(fromaddr="{from_address}",
                            sendToInterestedUsers=True,
                            extraRecipients={recipients},
                            lookup="{lookup}",
                            relayhost="{relay_host}", smtpPort={port},
                            smtpUser="{user}", buildSetSummary=True,
                            # addLogs=True,
                            smtpPassword="{password}")
c['services'].append(mail_notifier_service)

"#,
            recipients = format!("{:?}", self.extra_recipients),
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
                error!("There was a problem creating the mail notifier: '{}' section not specified. The build will continue with no mail notifier.", section);
                exit(1);
            }
        }

        let mut recipients = vec![];
        for recipient in yaml.get_section("extra-recipients").unwrap() {
            recipients.push(recipient.to_string());
        }

        let from_address = unwrap(&yaml, "from-address");
        let smtp_relay_host = unwrap(&yaml, "smtp-relay-host");
        let smtp_port = unwrap(&yaml, "smtp-port");
        let smtp_password = unwrap(&yaml, "smtp-password");
        let lookup = unwrap(&yaml, "lookup");

        Self::new(
            recipients,
            from_address,
            smtp_relay_host,
            smtp_port,
            lookup,
            smtp_password,
        )
    }
}

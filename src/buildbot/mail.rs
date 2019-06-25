
use crate::unwrap;
use rusty_yaml::Yaml;
use std::fmt::{Display, Error, Formatter};
use std::process::exit;


/// This object is responsible for building the `MailNotifier` object
/// in the buildbot master config. It contains the information for
/// authenticating an SMTP request to send email. This information is
/// sensitive and should be kept separate from the master yaml file
pub struct MailNotifier {
    extra_recipients: Vec<String>,
    from_address: String,
    smtp_relay_host: String,
    smtp_port: String,

    /// Identical to from_address
    smtp_user: String,

    smtp_password: String,
}

impl MailNotifier {
    pub fn new(
        extra_recipients: Vec<String>,
        from_address: String,
        smtp_relay_host: String,
        smtp_port: String,
        smtp_password: String,
    ) -> Self {
        Self {
            extra_recipients,
            from_address: from_address.clone(),
            smtp_relay_host,
            smtp_port,
            smtp_user: from_address,
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
                            relayhost="{relay_host}", smtpPort={port},
                            smtpUser="{user}", buildSetSummary=True, addLogs=True,
                            smtpPassword="{password}")
c['services'].append(mail_notifier_service)

"#,
            recipients = format!("{:?}", self.extra_recipients),
            from_address = self.from_address,
            relay_host = self.smtp_relay_host,
            password = self.smtp_password,
            user = self.smtp_user,
            port = self.smtp_port,
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

        Self::new(
            recipients,
            from_address,
            smtp_relay_host,
            smtp_port,
            smtp_password,
        )
    }
}

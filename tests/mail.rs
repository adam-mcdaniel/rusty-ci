extern crate rusty_yaml;
use rusty_yaml::Yaml;

extern crate rusty_ci;
use rusty_ci::MailNotifier;

#[test]
fn mailer_from_yaml() {
    let yaml = Yaml::from(
        r#"
extra-recipients:
  failure:
    - failure@gmail.com
  success:
    - success@gmail.com
  all:
    - all_tests@gmail.com

from-address: your-email-here@gmail.com
lookup: gmail.com
smtp-relay-host: smtp.gmail.com
smtp-port: 587
smtp-password: "p@$$w0rd""#,
    );

    let mailer = MailNotifier::from(yaml);
    let output = mailer.to_string();

    assert_eq!(
        output.trim(),
        "# The mail notifier responsible for all info\nall = reporters.MailNotifier(fromaddr=\"your-email-here@gmail.com\",\n                            sendToInterestedUsers=True,\n                            extraRecipients=[\"all_tests@gmail.com\"],\n                            lookup=\"gmail.com\",\n                            relayhost=\"smtp.gmail.com\", smtpPort=587,\n                            smtpUser=\"your-email-here@gmail.com\", buildSetSummary=True,\n                            # addLogs=True,\n                            mode=\"all\",\n                            smtpPassword=\"p@$$w0rd\")\nc[\'services\'].append(all)\n\n\n# The mail notifier responsible for failures\nfailures = reporters.MailNotifier(fromaddr=\"your-email-here@gmail.com\",\n                            sendToInterestedUsers=True,\n                            extraRecipients=[\"failure@gmail.com\"],\n                            lookup=\"gmail.com\",\n                            relayhost=\"smtp.gmail.com\", smtpPort=587,\n                            smtpUser=\"your-email-here@gmail.com\", buildSetSummary=True,\n                            # addLogs=True,\n                            mode=\"failing\",\n                            smtpPassword=\"p@$$w0rd\")\nc[\'services\'].append(failures)\n\n\n\n# The mail notifier responsible for successes\nsuccesses = reporters.MailNotifier(fromaddr=\"your-email-here@gmail.com\",\n                            sendToInterestedUsers=True,\n                            extraRecipients=[\"success@gmail.com\"],\n                            lookup=\"gmail.com\",\n                            relayhost=\"smtp.gmail.com\", smtpPort=587,\n                            smtpUser=\"your-email-here@gmail.com\", buildSetSummary=True,\n                            # addLogs=True,\n                            mode=\"passing\",\n                            smtpPassword=\"p@$$w0rd\")\nc[\'services\'].append(successes)".trim()
    );
}

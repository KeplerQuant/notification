use anyhow::Result;
use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, Mailbox, Mailboxes},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::notifier::Notifier;

#[derive(Debug, Clone)]
pub struct Email {
    from: Mailbox,
    mailer: SmtpTransport,
    recipients: Mailboxes,
}

impl Email {
    pub fn new(smtp_username: &str, smtp_password: &str, smtp_server: &str, from: &str) -> Self {
        let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());
        let mailer = SmtpTransport::relay(smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

        Self {
            mailer,
            from: from.parse().unwrap(),
            recipients: Mailboxes::new(),
        }
    }

    pub fn add_recipient(&mut self, recipient: String) {
        self.recipients.push(recipient.parse().unwrap());
    }
}

#[async_trait]
impl Notifier for Email {
    async fn send_message(&self, subject: &str, msg: &str) -> Result<()> {
        let message_builder = Message::builder()
            .from(self.from.to_owned())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN);

        for recipient in self.recipients.iter() {
            let email = message_builder
                .to_owned()
                .to(recipient.to_owned())
                .body(String::from(msg))?;

            self.mailer.send(&email)?;
        }

        Ok(())
    }
}

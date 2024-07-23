use anyhow::Result;
use async_trait::async_trait;
use lettre::{
    message::{header::ContentType, Mailbox, Mailboxes},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::notifier::Notifier;

/// `Email` struct to represent an email notification system.
/// This struct holds the SMTP transport, sender information, and recipients.
///
/// # Fields
///
/// * `from` - The sender's email address.
/// * `mailer` - The SMTP transport used to send the emails.
/// * `recipients` - The list of recipients' email addresses.
#[derive(Debug, Clone)]
pub struct Email {
    from: Mailbox,
    mailer: SmtpTransport,
    recipients: Mailboxes,
}

impl Email {
    /// Creates a new `Email` instance.
    ///
    /// # Arguments
    ///
    /// * `smtp_username` - The SMTP server username.
    /// * `smtp_password` - The SMTP server password.
    /// * `smtp_server` - The SMTP server address.
    /// * `from` - The sender's email address.
    ///
    /// # Returns
    ///
    /// * A new instance of `Email`.
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

    /// Adds a recipient to the email's recipient list.
    ///
    /// # Arguments
    ///
    /// * `recipient` - The email address of the recipient to be added.
    pub fn add_recipient(&mut self, recipient: String) {
        self.recipients.push(recipient.parse().unwrap());
    }
}

#[async_trait]
impl Notifier for Email {
    /// Sends a message to all the recipients.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject of the email.
    /// * `msg` - The body of the email.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns `Ok(())` if the email is sent successfully, otherwise returns an error.
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

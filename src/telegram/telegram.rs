use anyhow::Result;
use async_trait::async_trait;
use teloxide::{prelude::*, types::Recipient};

use crate::notifier::Notifier;

/// `Telegram` struct to represent a Telegram notification system.
/// This struct holds the Telegram bot and the list of recipients.
///
/// # Fields
///
/// * `bot` - The Telegram bot instance.
/// * `recipients` - The list of recipients' chat IDs.
#[derive(Debug, Clone)]
pub struct Telegram {
    bot: Bot,
    recipients: Vec<Recipient>,
}

impl Telegram {
    /// Creates a new `Telegram` instance.
    ///
    /// # Arguments
    ///
    /// * `token` - The Telegram bot token.
    ///
    /// # Returns
    ///
    /// * A new instance of `Telegram`.
    pub fn new(token: &str) -> Self {
        Self {
            bot: Bot::new(token),
            recipients: vec![],
        }
    }

    /// Adds a recipient to the Telegram's recipient list.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID of the recipient to be added.
    pub fn add_recipient(&mut self, chat_id: u64) {
        self.recipients.push(Recipient::from(UserId(chat_id)));
    }
}

#[async_trait]
impl Notifier for Telegram {
    /// Sends a message to all the recipients.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject of the message.
    /// * `msg` - The body of the message.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns `Ok(())` if the message is sent successfully, otherwise returns an error.
    async fn send_message(&self, subject: &str, msg: &str) -> Result<()> {
        let text = format!("{}\n{}", subject, msg);
        for recipient in &self.recipients {
            self.bot.send_message(recipient.clone(), &text).await?;
        }

        Ok(())
    }
}

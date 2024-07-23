use anyhow::Result;
use async_trait::async_trait;
use teloxide::{prelude::*, types::Recipient};

use crate::Notifier;

#[derive(Debug, Clone)]
pub struct Telegram {
    bot: Bot,
    recipients: Vec<Recipient>,
}

impl Telegram {
    pub fn new(token: &str) -> Self {
        Self {
            bot: Bot::new(token),
            recipients: vec![],
        }
    }

    pub fn add_recipient(&mut self, chat_id: u64) {
        self.recipients.push(Recipient::from(UserId(chat_id)));
    }
}

#[async_trait]
impl Notifier for Telegram {
    async fn send_message(&self, subject: &str, msg: &str) -> Result<()> {
        let text = format!("{}\n{}", subject, msg);
        for recipient in &self.recipients {
            self.bot.send_message(recipient.clone(), &text).await?;
        }

        Ok(())
    }
}

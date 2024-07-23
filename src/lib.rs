#[cfg(feature = "email")]
pub mod email;
pub mod notifier;
#[cfg(feature = "telegram")]
pub mod telegram;

use crate::notifier::Notifier;
use anyhow::Result;
use tokio::task::JoinSet;

#[derive(Debug, Clone)]
pub struct Notification<T: Notifier> {
    notifiers: Vec<T>,
}

impl<T: Notifier + Clone + 'static> Notification<T> {
    pub fn new() -> Self {
        Self { notifiers: vec![] }
    }

    pub fn add_notifier(&mut self, notifier: T) {
        self.notifiers.push(notifier);
    }

    pub async fn notify(&self, subject: &str, body: &str) -> Result<()> {
        let mut set = JoinSet::new();

        for notifier in &self.notifiers {
            let notifier = notifier.clone();
            let subject = subject.to_string();
            let body = body.to_string();

            set.spawn(async move { notifier.send_message(&subject, &body).await });
        }

        while let Some(res) = set.join_next().await {
            match res {
                Ok(a) => {
                    if let Err(e) = a {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}

#[cfg(feature = "email")]
pub mod email;
pub mod notifier;
#[cfg(feature = "telegram")]
pub mod telegram;

use std::sync::Arc;

use crate::notifier::Notifier;
use anyhow::Result;
use tokio::task::JoinSet;

#[derive(Clone, Default)]
pub struct Notification {
    notifiers: Vec<Arc<Box<dyn Notifier>>>,
}

impl Notification {
    pub fn new() -> Self {
        Self { notifiers: vec![] }
    }

    pub fn add_notifier(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(Arc::new(notifier));
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

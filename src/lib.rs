#[cfg(feature = "email")]
pub mod email;
#[cfg(feature = "telegram")]
pub mod telegram;

pub mod notifier;

use std::sync::Arc;

use crate::notifier::Notifier;
use anyhow::{Error, Result};
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

            set.spawn(async move {
                if let Err(e) = notifier.send_message(&subject, &body).await {
                    panic!("{}", e.to_string())
                }
            });
        }

        let mut results = vec![];
        while let Some(res) = set.join_next().await {
            results.push(res);
        }

        for result in results {
            if let Err(e) = result {
                return Err(Error::new(e));
            }
        }

        Ok(())
    }
}

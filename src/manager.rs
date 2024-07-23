use crate::notifier::Notifier;
use anyhow::Result;
use tokio::task::JoinSet;

#[derive(Debug, Clone)]
pub struct NotifierManager<T: Notifier> {
    notifiers: Vec<T>,
}

impl<T: Notifier + Clone + 'static> NotifierManager<T> {
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

        while let Some(result) = set.join_next().await {
            match result {
                Ok(result) => {
                    if let Err(e) = result {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}

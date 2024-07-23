use std::sync::Arc;

use anyhow::Result;
use tokio::task::JoinSet;

use crate::notifier::Notifier;

/// `NotifierManager` struct to manage and coordinate multiple notifiers.
/// This struct holds a list of notifiers and provides methods to add new notifiers and send notifications.
///
/// # Fields
///
/// * `notifiers` - A vector of notifiers implementing the `Notifier` trait.
#[derive(Clone)]
pub struct NotifierManager {
    notifiers: Vec<Arc<Box<dyn Notifier>>>,
}

impl NotifierManager {
    /// Creates a new `NotifierManager` instance.
    ///
    /// # Returns
    ///
    /// * A new instance of `NotifierManager`.
    pub fn new() -> Self {
        Self { notifiers: vec![] }
    }

    /// Adds a notifier to the manager's list of notifiers.
    ///
    /// # Arguments
    ///
    /// * `notifier` - A notifier implementing the `Notifier` trait to be added.
    pub fn add_notifier(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(Arc::new(notifier));
    }

    /// Sends a notification using all the notifiers.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject of the notification.
    /// * `body` - The body of the notification.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns `Ok(())` if all notifications are sent successfully, otherwise returns an error.
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

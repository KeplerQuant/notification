use anyhow::Result;
use async_trait::async_trait;

/// The `Notifier` trait defines the interface for sending messages.
/// Any struct implementing this trait must provide an implementation
/// for the `send_message` method, which sends a message with a given
/// subject and body.
#[async_trait]
pub trait Notifier: Send + Sync {
    async fn send_message(&self, subject: &str, msg: &str) -> Result<()>;
}

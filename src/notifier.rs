use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn send_message(&self, subject: &str, msg: &str) -> Result<()>;
}

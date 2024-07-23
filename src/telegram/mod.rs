/// This module provides functionality for sending messages via Telegram.
/// It defines a `Telegram` struct which can be used to configure Telegram API settings,
/// add recipients, and send messages.
mod telegram;

pub use telegram::Telegram;

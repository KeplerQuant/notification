/// This module provides functionality for sending emails using the `lettre` crate.
/// It defines an `Email` struct which can be used to configure SMTP settings,
/// add recipients, and send email messages.
#[cfg(feature = "email")]
pub mod email;

/// This module provides functionality for sending messages via Telegram.
/// It defines a `Telegram` struct which can be used to configure Telegram API settings,
/// add recipients, and send messages.
#[cfg(feature = "telegram")]
pub mod telegram;

/// This module provides the `Manager` struct which coordinates and manages
/// different types of notifiers such as email and telegram.
pub mod manager;

/// This module defines the `Notifier` trait which outlines the required methods
/// for any notifier implementation, such as sending messages.
pub mod notifier;

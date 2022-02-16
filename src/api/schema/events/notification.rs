use std::fmt::Display;

use async_graphql::{Object, SimpleObject, Union};

#[derive(Debug, Clone, PartialEq, SimpleObject)]
/// A component was found that matched the provided pattern
pub struct Matched {
    #[graphql(skip)]
    message: String,
    /// Pattern that raised the notification
    pub pattern: String,
}

impl Matched {
    pub fn new(pattern: String) -> Self {
        Self {
            message: format!("[tap] Pattern '{}' successfully matched.", pattern),
            pattern,
        }
    }
}

#[derive(Debug, Clone, PartialEq, SimpleObject)]
/// There isn't currently a component that matches this pattern
pub struct NotMatched {
    #[graphql(skip)]
    message: String,
    /// Pattern that raised the notification
    pub pattern: String,
}

impl NotMatched {
    pub fn new(pattern: String) -> Self {
        Self {
            message: format!(
                "[tap] Pattern '{}' failed to match: will retry on configuration reload.",
                pattern
            ),
            pattern,
        }
    }
}

#[derive(Debug, Clone, PartialEq, SimpleObject)]
/// The pattern matched source(s) which cannot be tapped for inputs or sink(s)
/// which cannot be tapped for outputs
pub struct InvalidMatch {
    #[graphql(skip)]
    message: String,
    /// Pattern that raised the notification
    pattern: String,
    /// Any invalid matches for the pattern
    invalid_matches: Vec<String>,
}

impl InvalidMatch {
    pub fn new(message: String, pattern: String, invalid_matches: Vec<String>) -> Self {
        Self {
            message,
            pattern,
            invalid_matches,
        }
    }
}

#[derive(Union, Debug, Clone, PartialEq)]
/// A notification regarding events observation
pub enum Notification {
    Matched(Matched),
    NotMatched(NotMatched),
    InvalidMatch(InvalidMatch),
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: &str = match self {
            Notification::Matched(n) => n.message.as_ref(),
            Notification::NotMatched(n) => n.message.as_ref(),
            Notification::InvalidMatch(n) => n.message.as_ref(),
        };
        write!(f, "{}", message)
    }
}

/// This wrapper struct hoists `message` up from [`Notification`] for a more
/// natural querying experience. While ideally [`Notification`] would be a
/// GraphQL interface, there were issues directly nesting an interface into the
/// union of [`super::OutputEventsPayload`]
#[derive(Debug, Clone)]
pub struct EventNotification {
    pub notification: Notification,
}

#[Object]
impl EventNotification {
    /// Notification details
    async fn notification(&self) -> &Notification {
        &self.notification
    }

    /// The human-readable message associated with the notification
    async fn message(&self) -> String {
        self.notification.to_string()
    }
}

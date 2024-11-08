use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use storeful::Context;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingLog {
    pub timestamp: Option<DateTime<Utc>>,
    pub context: Context,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub timestamp: DateTime<Utc>,
    pub context: Context,
    pub message: String,
}

impl From<IncomingLog> for Log {
    fn from(incoming: IncomingLog) -> Self {
        Log {
            timestamp: incoming.timestamp.unwrap_or(Utc::now()),
            context: incoming.context,
            message: incoming.message,
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.timestamp.to_rfc3339_opts(SecondsFormat::Nanos, true),
            self.context,
            self.message
        )
    }
}

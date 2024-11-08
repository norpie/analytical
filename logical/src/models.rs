use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use storeful::Context;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingLog {
    pub timestamp: Option<i64>,
    pub context: Context,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub timestamp: i64,
    pub context: Context,
    pub message: String,
}

impl From<IncomingLog> for Log {
    fn from(incoming: IncomingLog) -> Self {
        Log {
            // Breaks after `2262-04-11T23:47:16.854775807`, but I doubt my code will be running by
            // then. If it is, I'll be sure to fix it then :).
            // (https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.timestamp_nanos_opt)
            timestamp: incoming
                .timestamp
                .unwrap_or(Utc::now().timestamp_nanos_opt().unwrap()),
            context: incoming.context,
            message: incoming.message,
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = DateTime::from_timestamp_nanos(self.timestamp).to_utc();
        write!(
            f,
            "{} {} {}",
            date.to_rfc3339_opts(SecondsFormat::Nanos, true),
            self.context,
            self.message
        )
    }
}

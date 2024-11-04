use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use storeful::Labels;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingLog {
    pub timestamp: Option<i64>,
    pub labels: Labels,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub timestamp: i64,
    pub labels: Labels,
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
            labels: incoming.labels,
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
            self.labels,
            self.message
        )
    }
}

#[cfg(test)]
mod test {

    use storeful::Label;

    use super::*;

    #[test]
    pub fn test() {
        let log: Log = IncomingLog {
            timestamp: None,
            labels: Labels(vec![
                Label {
                    key: "region".into(),
                    value: "eu-west-1".into(),
                },
                Label {
                    key: "machine".into(),
                    value: "i-1234567890abcdef0".into(),
                },
                Label {
                    key: "module".into(),
                    value: module_path!().into(),
                },
                Label {
                    key: "function".into(),
                    value: "test".into(),
                },
                Label {
                    key: "severity".into(),
                    value: "info".into(),
                },
            ]),
            message: "Hello, world!".into(),
        }
        .into();
        println!("{}", log);
    }
}

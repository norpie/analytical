use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use storeful::Labels;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingMetric {
    pub timestamp: Option<i64>,
    pub name: String,
    pub labels: Labels,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub timestamp: i64,
    pub name: String,
    pub labels: Labels,
    pub value: f64,
}

impl From<IncomingMetric> for Metric {
    fn from(incoming: IncomingMetric) -> Self {
        Metric {
            timestamp: incoming
                .timestamp
                .unwrap_or(Utc::now().timestamp_nanos_opt().unwrap()),
            name: incoming.name,
            labels: incoming.labels,
            value: incoming.value,
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = DateTime::from_timestamp_nanos(self.timestamp).to_utc();
        write!(
            f,
            "{} {}{} {}",
            date.to_rfc3339_opts(SecondsFormat::Nanos, true),
            self.name,
            self.labels,
            self.value
        )
    }
}

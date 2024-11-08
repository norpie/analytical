use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use storeful::Context;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingMetric {
    pub timestamp: Option<DateTime<Utc>>,
    pub name: String,
    pub context: Context,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub context: Context,
    pub value: f64,
}

impl From<IncomingMetric> for Metric {
    fn from(incoming: IncomingMetric) -> Self {
        Metric {
            timestamp: incoming.timestamp.unwrap_or(Utc::now()),
            name: incoming.name,
            context: incoming.context,
            value: incoming.value,
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{} {}",
            self.timestamp.to_rfc3339_opts(SecondsFormat::Nanos, true),
            self.name,
            self.context,
            self.value
        )
    }
}

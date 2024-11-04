use std::fmt::Display;

use chrono::{DateTime, SecondsFormat};
use serde::{Deserialize, Serialize};
use storeful::Labels;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trace {
    pub labels: Labels,
    pub events: TraceEvents,
}

impl Display for Trace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.labels, self.events)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceEvents(pub Vec<TraceEvent>);

impl Display for TraceEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let events = self
            .0
            .iter()
            .map(|event| format!("{}", event))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{{{}}}", events)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceEvent {
    pub name: String,
    pub event_type: TraceEventType,
    pub timestamp: i64,
}

impl Display for TraceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = DateTime::from_timestamp_nanos(self.timestamp).to_utc();
        write!(
            f,
            "{} {} {}",
            date.to_rfc3339_opts(SecondsFormat::Nanos, true),
            self.name,
            self.event_type,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TraceEventType {
    Start,
    End,
    Annotation,
}

impl Display for TraceEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceEventType::Start => write!(f, "Start"),
            TraceEventType::End => write!(f, "End"),
            TraceEventType::Annotation => write!(f, "Annotation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use storeful::{Label, Labels};

    #[test]
    fn test() {
        let trace = Trace {
            labels: Labels(vec![Label {
                key: "key1".to_string(),
                value: "value1".to_string(),
            }]),
            events: TraceEvents(vec![
                TraceEvent {
                    name: "event1".to_string(),
                    event_type: TraceEventType::Start,
                    timestamp: Utc::now().timestamp_nanos_opt().unwrap() + 1,
                },
                TraceEvent {
                    name: "random_event".to_string(),
                    event_type: TraceEventType::Annotation,
                    timestamp: Utc::now().timestamp_nanos_opt().unwrap() + 2,
                },
                TraceEvent {
                    name: "event1".to_string(),
                    event_type: TraceEventType::End,
                    timestamp: Utc::now().timestamp_nanos_opt().unwrap() + 3,
                },
            ]),
        };
        println!("{}", trace);
    }
}

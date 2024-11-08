use serde::{Deserialize, Serialize};
use storeful::{ContextValue, Context, Query};

impl Query for MetricQuery {
    fn from_str(s: &str) -> Self {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricQuery {
    pub name: Option<String>,
    pub timestamp_start: Option<i64>,
    pub timestamp_end: Option<i64>,
    pub context: Option<Context>,
}

impl MetricQuery {
    pub fn empty() -> Self {
        Self {
            name: None,
            timestamp_start: None,
            timestamp_end: None,
            context: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_timestamp_start(mut self, timestamp_start: i64) -> Self {
        self.timestamp_start = Some(timestamp_start);
        self
    }

    pub fn with_timestamp_end(mut self, timestamp_end: i64) -> Self {
        self.timestamp_end = Some(timestamp_end);
        self
    }

    pub fn with_context_value(mut self, context_value: ContextValue) -> Self {
        if let Some(mut context) = self.context {
            context.0.push(context_value);
            self.context = Some(context);
        } else {
            self.context = Some(Context(vec![context_value]));
        }
        self
    }
}

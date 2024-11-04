use storeful::{Label, Labels};

#[derive(Debug)]
pub struct MetricQuery {
    pub name: Option<String>,
    pub timestamp_start: Option<i64>,
    pub timestamp_end: Option<i64>,
    pub labels: Option<Labels>,
}

impl MetricQuery {
    pub fn empty() -> Self {
        Self {
            name: None,
            timestamp_start: None,
            timestamp_end: None,
            labels: None,
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

    pub fn with_label(mut self, label: Label) -> Self {
        if let Some(mut labels) = self.labels {
            labels.0.push(label);
            self.labels = Some(labels);
        } else {
            self.labels = Some(Labels(vec![label]));
        }
        self
    }
}

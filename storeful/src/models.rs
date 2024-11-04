use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingBody<T> {
    pub value: Option<T>,
    pub values: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub key: String,
    pub value: String,
}

/// `key1="value1"`
impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.key, self.value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Labels(pub Vec<Label>);

/// `{ key1="value1", key2="value2" }`
impl Display for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let labels = self
            .0
            .iter()
            .map(|label| format!("{}", label))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{{{}}}", labels)
    }
}

impl Labels {
    pub fn to_key_string(&self) -> String {
        self.0
            .iter()
            .map(|label| format!("{}=\"{}\"", label.key, label.value))
            .collect::<Vec<String>>()
            .join(",")
    }
}

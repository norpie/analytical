use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingBody<T> {
    pub value: Option<T>,
    pub values: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextValue {
    pub key: String,
    pub value: String,
}

/// `key1="value1"`
impl Display for ContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.key, self.value)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Context(pub Vec<ContextValue>);

/// `{ key1="value1", key2="value2" }`
impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context = self
            .0
            .iter()
            .map(|context_value| format!("{}", context_value))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{{{}}}", context)
    }
}

impl Context {
    pub fn add_value(&mut self, key: &str, value: &str) {
        self.0.push(ContextValue { key: key.into(), value: value.into() });
    }

    pub fn with_value(mut self, key: &str, value: &str) -> Self {
        self.add_value(key, value);
        self
    }

    pub fn to_key_string(&self) -> String {
        self.0
            .iter()
            .map(|context_value| format!("{}=\"{}\"", context_value.key, context_value.value))
            .collect::<Vec<String>>()
            .join(",")
    }
}

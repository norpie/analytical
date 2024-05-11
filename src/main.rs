#![forbid(missing_docs)]
//! # Metrical
//! > A simple metrics database.
//!
//! Metrical is a simple metrics database that stores metrics on disk with rocksdb.
//!
//! ## Format
//!
//! The data stored in the database is as follows:
//! * `metric` - The name of the metric.
//! * `key` - The key of the metric. This is used to identify the metric.
//! * `timestamp` - The timestamp of the metric.
//! * `value` - A float value of the metric.
//!
//! ### Examples
//!
//! #### Variable Metrics
//!
//! ```json
//! {
//!    "metric": "cpu",
//!    "key": "backend-server1",
//!    "timestamp": 1234567890,
//!    "value": 0.532
//! }
//! ```
//!
//! #### Boolean Metrics
//!
//! ```json
//! {
//!    "metric": "connected_to_db",
//!    "key": "database-server1"
//!    "timestamp": 1234567890,
//!    "value": 1
//! }
//! ```

use serde::{Deserialize, Serialize};

extern crate rocksdb;

/// # Metrical
/// The main struct that is used to interact with the database.
struct Metrical {
    db: rocksdb::DB,
}

/// # Metric
/// A metric is a single data point that is stored in the database.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
struct Metric {
    name: String,
    key: String,
    timestamp: u64,
    value: f64,
}

fn main() {
    println!("Hello, world!");
}

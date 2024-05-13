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
//!
//! ## Storage
//!
//! Each `metric` has its own `store` in the database. Each `key` has a "table" in the `store`.
//! This allows for easy querying of metrics.

use std::path::PathBuf;

use anyhow::{Result, Error};
use clap::{command, Parser};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

extern crate rocksdb;

/// The global instance of the Metrical struct.
static INSTANCE: OnceCell<Metrical> = OnceCell::new();

/// # Metrical
/// The main struct that is used to interact with the database.
#[derive(Debug)]
struct Metrical {
    db: rocksdb::DB,
}

impl Metrical {
    /// Create a new Metrical instance.
    fn new(db_path: PathBuf) -> Result<Self, Error> {
        let db = rocksdb::DB::open_default(db_path)?;
        Ok(Self { db })
    }

    fn db(&mut self) -> &mut rocksdb::DB {
        &mut self.db
    }
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the database.
    #[clap(
        long,
        default_value = "/etc/metrical/default.db"
    )]
    db_path: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    println!("Opening database at: {:?}", args.db_path);
    INSTANCE.set(Metrical::new(args.db_path)?).map_err(|_| anyhow::anyhow!("Failed to set Metrical instance"))?;
    Ok(())
}

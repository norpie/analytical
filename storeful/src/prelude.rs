use std::{io, sync::PoisonError};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, StorefulError>;

#[derive(Error, Debug)]
pub enum StorefulError {
    #[error("misc error")]
    Misc,

    #[error("error parsing {1} at position {0}")]
    Parse(u32, char),

    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),

    #[error("unknown Storeful error")]
    Unknown,

    #[error("failed to open database: {0}")]
    Open(String),

    #[error("json error")]
    Json(#[from] serde_json::Error),

    #[error("bincode error")]
    Bincode(#[from] bincode::Error),

    // #[error("rocksdb error")]
    // Rocks(#[from] rocksdb::Error),
    #[error("sled error")]
    Sled(#[from] sled::Error),

    #[error("from utf8 error")]
    StringUtf8(#[from] std::string::FromUtf8Error),

    #[error("from utf8 error")]
    StrUtf8(#[from] std::str::Utf8Error),

    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("column family not found: {0}")]
    ColumnFamilyNotFound(String),

    #[error("batch already started")]
    BatchAlreadyStarted,
    #[error("batch not started")]
    BatchNotStarted,

    #[error("invalid query range")]
    InvalidQueryRange,

    #[error("lock poisoned")]
    LockPoisoned,
}

impl<T> From<PoisonError<T>> for StorefulError {
    fn from(_: PoisonError<T>) -> Self {
        StorefulError::LockPoisoned
    }
}

use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, StorefulError>;

#[derive(Error, Debug)]
pub enum StorefulError {
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

    #[error("rocksdb error")]
    Rocks(#[from] rocksdb::Error),

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
}
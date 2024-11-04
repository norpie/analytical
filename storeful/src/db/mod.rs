use std::collections::HashSet;

use crate::prelude::*;

pub mod rocksdb;

pub trait BackendDatabase {
    fn start_batch(&mut self) -> Result<()>;
    fn commit_batch(&mut self) -> Result<()>;

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()>;
    fn get(&self, key: &str) -> Result<Option<Box<[u8]>>>;
    fn get_multi(&self, keys: &HashSet<Box<[u8]>>) -> Result<Vec<Box<[u8]>>>;

    fn create_index(&mut self, cf: &str, primary: &str, key: &str) -> Result<()>;

    fn query_timestamp_index(
        &self,
        timestamp_start: Option<i64>,
        timestamp_end: Option<i64>,
    ) -> Result<HashSet<Box<[u8]>>>;
    fn query_index(&self, cf: &str, index_key: &str) -> Result<HashSet<Box<[u8]>>>;
}

pub struct Storeful {
    pub backend: Box<dyn BackendDatabase>,
}

impl Storeful {
    pub fn new(backend: Box<dyn BackendDatabase>) -> Self {
        Self { backend }
    }
}

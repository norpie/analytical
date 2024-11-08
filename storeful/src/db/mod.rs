use std::collections::HashSet;

use crate::prelude::*;

// pub mod rocksdb;
pub mod sled;

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

pub struct Storeful<B>
where
    B: BackendDatabase + Send + Sync,
{
    pub backend: B,
}

impl<B> Storeful<B>
where
    B: BackendDatabase + Send + Sync,
{
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

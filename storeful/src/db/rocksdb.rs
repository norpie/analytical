use crate::prelude::*;

use std::{collections::HashSet, path::PathBuf};

use chrono::Utc;
use rocksdb::{BlockBasedOptions, ColumnFamily, Direction, IteratorMode, Options, WriteBatch, DB};

use super::BackendDatabase;

pub struct RocksDBBackend {
    pub db: DB,
    pub batch: Option<WriteBatch>,
}

impl BackendDatabase for RocksDBBackend {
    fn start_batch(&mut self) -> Result<()> {
        if self.batch.is_some() {
            return Err(StorefulError::BatchAlreadyStarted);
        }
        self.batch = Some(WriteBatch::default());
        Ok(())
    }

    fn commit_batch(&mut self) -> Result<()> {
        if let Some(batch) = self.batch.take() {
            self.db.write(batch)?;
        }
        Ok(())
    }

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        if let Some(batch) = self.batch.as_mut() {
            batch.put(key, value);
        } else {
            self.db.put(key, value)?;
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<Box<[u8]>>> {
        let result = self.db.get(key)?;
        Ok(result.map(|value| value.to_vec().into_boxed_slice()))
    }

    fn get_multi(&self, keys: &HashSet<Box<[u8]>>) -> Result<Vec<Box<[u8]>>> {
        Ok(self
            .db
            .multi_get(keys)
            .into_iter()
            .flatten()
            .flatten()
            .map(|item| item.into_boxed_slice())
            .collect())
    }

    fn create_index(&mut self, cf: &str, primary: &str, key: &str) -> Result<()> {
        self.create_index_cf(cf, primary, key)
    }

    fn query_timestamp_index(
        &self,
        timestamp_start: Option<i64>,
        timestamp_end: Option<i64>,
    ) -> Result<HashSet<Box<[u8]>>> {
        let mut results = HashSet::new();
        let cf = self
            .db
            .cf_handle("timestamp")
            .expect("timestamp cf not found");

        // Prepare iterator based on start condition
        let iter = if let Some(start) = timestamp_start {
            self.db.iterator_cf(
                cf,
                IteratorMode::From(
                    format!("timestamp:{:0>20}|", start).as_bytes(),
                    Direction::Forward,
                ),
            )
        } else {
            self.db.iterator_cf(cf, IteratorMode::Start)
        };

        for result in iter {
            let (key_slice, primary) = result?;

            // Directly parse timestamp from key bytes to avoid unnecessary conversions
            let timestamp_str = &key_slice[10..30]; // Slice to where timestamp is in the key
            let timestamp: i64 = std::str::from_utf8(timestamp_str)?.parse()?;

            // Check range
            if let Some(start) = timestamp_start {
                if timestamp < start {
                    continue;
                }
            }
            if let Some(end) = timestamp_end {
                if timestamp > end {
                    break;
                }
            }

            // Convert primary key only once confirmed within range
            results.insert(primary);
        }

        Ok(results)
    }

    fn query_index(&self, cf: &str, index_key: &str) -> Result<HashSet<Box<[u8]>>> {
        let handle = self
            .db
            .cf_handle(cf)
            .unwrap_or_else(|| panic!("{} column family not found", cf));
        self.query_index_cf(handle, index_key)
    }
}

impl RocksDBBackend {
    pub fn open(path: &PathBuf, indexes: Vec<&'static str>) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        let mut block_opts = BlockBasedOptions::default();
        block_opts.set_bloom_filter(10.0, false);
        opts.set_block_based_table_factory(&block_opts);
        let db =
            DB::open_cf(&opts, path, indexes).map_err(|e| StorefulError::Open(e.to_string()))?;
        Ok(Self { db, batch: None })
    }

    pub fn create_index_cf(&mut self, cf: &str, primary: &str, key: &str) -> Result<()> {
        let handle = self.db.cf_handle(cf).expect("cf not found");
        if let Some(batch) = self.batch.as_mut() {
            batch.put_cf(handle, key, primary);
        } else {
            self.db.put_cf(handle, key, primary)?;
        }
        Ok(())
    }

    pub fn query_index_cf(&self, cf: &ColumnFamily, index_key: &str) -> Result<HashSet<Box<[u8]>>> {
        let mut results = HashSet::new();
        let iter = self.db.prefix_iterator_cf(cf, index_key);
        for iter_result in iter {
            let (key, primary) = iter_result?;
            if !key.starts_with(index_key.as_bytes()) {
                break;
            }
            results.insert(primary);
        }
        Ok(results)
    }
}

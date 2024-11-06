use sled::{Batch, Tree};

use crate::{prelude::*, BackendDatabase};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub struct SledBackend {
    db: sled::Db,
    master_key: String,
    trees: HashMap<String, Tree>,
    batch: Option<Batch>,
}

impl SledBackend {
    pub fn open(path: &PathBuf, master_key: String, tree_names: &[&'static str]) -> Result<Self> {
        let config = sled::Config::new().path(path);
        let db = config.open()?;
        let mut trees = HashMap::new();
        for tree_name in tree_names {
            let tree = db.open_tree(tree_name)?;
            trees.insert(format!("{}:{}", &master_key, tree_name), tree);
        }
        Ok(Self {
            db,
            master_key,
            trees,
            batch: None,
        })
    }
}

impl BackendDatabase for SledBackend {
    fn start_batch(&mut self) -> Result<()> {
        if self.batch.is_some() {
            return Err(StorefulError::BatchAlreadyStarted);
        } else {
            self.batch = Some(sled::Batch::default());
        }
        Ok(())
    }

    fn commit_batch(&mut self) -> Result<()> {
        if let Some(batch) = self.batch.take() {
            Ok(self.db.apply_batch(batch)?)
        } else {
            Err(StorefulError::BatchNotStarted)
        }
    }

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        if let Some(batch) = self.batch.as_mut() {
            batch.insert(key, value);
        } else {
            self.db.insert(key, value)?;
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<Box<[u8]>>> {
        let result = self.db.get(key)?;
        Ok(result.map(|value| value.to_vec().into_boxed_slice()))
    }

    fn get_multi(&self, keys: &std::collections::HashSet<Box<[u8]>>) -> Result<Vec<Box<[u8]>>> {
        let mut result = Vec::new();
        for key in keys {
            let value = self.db.get(key)?;
            if let Some(value) = value {
                result.push(value.to_vec().into_boxed_slice());
            }
        }
        Ok(result)
    }

    fn create_index(&mut self, tree: &str, primary_key: &str, key: &str) -> Result<()> {
        let tree = self
            .trees
            .get(&format!("{}:{}", &self.master_key, tree))
            .unwrap();
        tree.insert(key, primary_key)?;
        Ok(())
    }

    fn query_timestamp_index(
        &self,
        timestamp_start: Option<i64>,
        timestamp_end: Option<i64>,
    ) -> Result<std::collections::HashSet<Box<[u8]>>> {
        let tree = self
            .trees
            .get(&format!("{}:{}", &self.master_key, "timestamp"))
            .unwrap();
        let mut results = HashSet::new();
        for result in tree.iter() {
            let (key, value) = result?;

            // Directly parse timestamp from key bytes to avoid unnecessary conversions
            let timestamp_str = &key[10..30]; // Slice to where timestamp is in the key
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
            results.insert(value.to_vec().into_boxed_slice());
        }
        Ok(results)
    }

    fn query_index(
        &self,
        tree: &str,
        index_key: &str,
    ) -> Result<std::collections::HashSet<Box<[u8]>>> {
        // iterate with prefix of index_key
        let tree = self
            .trees
            .get(&format!("{}:{}", &self.master_key, tree))
            .unwrap();
        let mut result = HashSet::new();
        for item in tree.scan_prefix(index_key) {
            let (_, value) = item?;
            result.insert(value.to_vec().into_boxed_slice());
        }
        Ok(result)
    }
}

use std::{collections::HashSet, path::PathBuf};

use chrono::Utc;
use models::Metric;
use query::MetricQuery;
use rand::prelude::SliceRandom;
use storeful::{intersect, prelude::*, rocksdb::RocksDBBackend, Label, Labels, Storeful};

mod models;
mod query;

struct Metrical {
    storeful: Storeful,
}

impl Metrical {
    pub fn new(storeful: Storeful) -> Self {
        Self { storeful }
    }

    pub fn create_metric(&mut self, metric: Metric) -> Result<()> {
        let timestamp = format!("{:0>20}", metric.timestamp);
        let primary = format!(
            "{}|{}|{}",
            metric.name,
            timestamp,
            metric.labels.to_key_string()
        );
        self.storeful
            .backend
            .put(&primary, &bincode::serialize(&metric)?)?;

        self.storeful.backend.create_index(
            "timestamp",
            &primary,
            &format!("timestamp:{:0>20}|{}", timestamp, primary),
        )?;

        self.storeful.backend.create_index(
            "name",
            &primary,
            &format!("name|{}|{}", metric.name, primary),
        )?;

        for label in metric.labels.0 {
            self.storeful.backend.create_index(
                "labels",
                &primary,
                &format!("label:{}:{}|{}", label.key, label.value, &primary),
            )?;
        }

        Ok(())
    }

    pub fn create_metrics(&mut self, metrics: Vec<Metric>) -> Result<()> {
        self.storeful.backend.start_batch()?;
        for metric in metrics {
            self.create_metric(metric)?;
        }
        self.storeful.backend.commit_batch()?;
        Ok(())
    }

    pub fn get_metrics(&self, primaries: &HashSet<Box<[u8]>>) -> Result<Vec<Metric>> {
        let start = Utc::now();
        let results = self.storeful.backend.get_multi(primaries)?;
        let after_get = Utc::now();
        let mut metrics = Vec::new();
        for result in results {
            metrics.push(bincode::deserialize(&result)?);
        }
        let after_deserialize = Utc::now();
        println!(
            "Getting metrics took {}us",
            (after_get - start).num_microseconds().unwrap()
        );
        println!(
            "Deserializing metrics took {}us",
            (after_deserialize - after_get).num_microseconds().unwrap()
        );
        Ok(metrics)
    }

    pub fn query(&self, query: MetricQuery) -> Result<Vec<Metric>> {
        let mut primaries = HashSet::new();
        let start = Utc::now();
        if let Some(name) = query.name {
            let name_key = format!("name|{}|", name);
            let name_primaries = self.storeful.backend.query_index("name", &name_key)?;
            let after_name_query = Utc::now();
            intersect(&mut primaries, name_primaries);
            let after_intersect = Utc::now();
            println!(
                "intersecting name took {}us",
                (after_intersect - after_name_query).num_microseconds().unwrap()
            );
        }
        let after_name = Utc::now();
        if query.timestamp_start.is_some() || query.timestamp_end.is_some() {
            let timestamp_primaries = self
                .storeful
                .backend
                .query_timestamp_index(query.timestamp_start, query.timestamp_end)?;
            intersect(&mut primaries, timestamp_primaries);
        }
        let after_timestamp = Utc::now();
        if let Some(labels) = query.labels {
            for label in labels.0 {
                let label_key = format!("label:{}:{}|", label.key, label.value);
                let label_primaries = self.storeful.backend.query_index("labels", &label_key)?;
                intersect(&mut primaries, label_primaries);
            }
        }
        let after_labels = Utc::now();
        println!(
            "Querying name took {}us",
            (after_name - start).num_microseconds().unwrap()
        );
        println!(
            "Querying timestamp took {}us",
            (after_timestamp - after_name).num_microseconds().unwrap()
        );
        println!(
            "Querying labels took {}us",
            (after_labels - after_timestamp).num_microseconds().unwrap()
        );
        self.get_metrics(&primaries)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Args::default();

    let path = PathBuf::from("./test.db");
    let rocksdb = RocksDBBackend::open(&path, vec!["name", "timestamp", "labels"])?;
    let storeful = Storeful::new(Box::new(rocksdb));
    let mut metrical = Metrical::new(storeful);

    let start_time = Utc::now();

    let mut metrics = vec![];
    let metric_count = 30000;
    for i in 0..metric_count {
        // Random value between 0 and 1
        let random_value = rand::random::<f64>();
        // Random value between 0 and 5 as int
        let random_timestamp = i;
        let name_choices = ["cpu_usage", "memory_usage", "disk_usage"];
        let host_choices = ["localhost", "server1", "server2", "server3", "server4"];
        let region_choices = ["us-west", "us-east", "eu-west", "eu-east"];
        let metric = Metric {
            name: name_choices
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            timestamp: random_timestamp,
            value: random_value,
            labels: Labels(vec![
                Label {
                    key: "host".into(),
                    value: host_choices
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                },
                Label {
                    key: "region".into(),
                    value: region_choices
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                },
            ]),
        };
        metrics.push(metric);
    }

    metrical.create_metrics(metrics)?;

    let written_time = Utc::now();

    // Do a query by all different fields
    let query = MetricQuery::empty()
        .with_name("memory_usage".into())
        .with_timestamp_start(0)
        .with_timestamp_end(10000)
        .with_label(Label {
            key: "host".into(),
            value: "localhost".into(),
        })
        .with_label(Label {
            key: "region".into(),
            value: "us-west".into(),
        });

    let results = metrical.query(query)?;

    // dbg!(&results);

    let queried_time = Utc::now();

    println!(
        "Queried duration: {}ms",
        (queried_time - written_time).num_milliseconds()
    );
    println!(
        "Total duration, writing {} and querying {} records: {}ms",
        metric_count,
        results.len(),
        (queried_time - start_time).num_milliseconds()
    );
    Ok(())
}

use std::collections::HashSet;

use crate::models::Metric;
use crate::query::MetricQuery;
use chrono::Utc;
use storeful::{intersect, prelude::*, BackendDatabase, ModelEndpoints, Storeful};

pub struct Metrical<B>
where
    B: BackendDatabase,
{
    storeful: Storeful<B>,
}

impl<B> Metrical<B>
where
    B: BackendDatabase,
{
    pub fn new(storeful: Storeful<B>) -> Self {
        Self { storeful }
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
}

impl<B> ModelEndpoints<Metric, MetricQuery> for Metrical<B>
where
    B: BackendDatabase,
{
    async fn post(&mut self, metric: Metric) -> Result<()> {
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
            &format!("timestamp|{:0>20}|{}", timestamp, primary),
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
                &format!("label|{}:{}|{}", label.key, label.value, &primary),
            )?;
        }

        Ok(())
    }

    async fn post_multi(&mut self, input: Vec<Metric>) -> Result<()> {
        self.storeful.backend.start_batch()?;
        for metric in input {
            self.post(metric).await?;
        }
        self.storeful.backend.commit_batch()?;
        Ok(())
    }

    async fn query(&mut self, query: MetricQuery) -> Result<Vec<Metric>> {
        let mut primaries = HashSet::new();
        if let Some(name) = query.name {
            let name_key = format!("name|{}|", name);
            let name_primaries = self.storeful.backend.query_index("name", &name_key)?;
            let after_name_query = Utc::now();
            intersect(&mut primaries, name_primaries);
            let after_intersect = Utc::now();
            println!(
                "intersecting name took {}us",
                (after_intersect - after_name_query)
                    .num_microseconds()
                    .unwrap()
            );
        }
        if query.timestamp_start.is_some() || query.timestamp_end.is_some() {
            let timestamp_primaries = self
                .storeful
                .backend
                .query_timestamp_index(query.timestamp_start, query.timestamp_end)?;
            intersect(&mut primaries, timestamp_primaries);
        }
        if let Some(labels) = query.labels {
            for label in labels.0 {
                let label_key = format!("label|{}:{}|", label.key, label.value);
                let label_primaries = self.storeful.backend.query_index("labels", &label_key)?;
                intersect(&mut primaries, label_primaries);
            }
        }
        self.get_metrics(&primaries)
    }
}

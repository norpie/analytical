use std::collections::HashSet;

use crate::models::Metric;
use crate::query::MetricQuery;
use storeful::{intersect, prelude::*, BackendDatabase, ModelEndpoints, Storeful};

pub struct Metrical<B>
where
    B: BackendDatabase + Send + Sync,
{
    storeful: Storeful<B>,
}

impl<B> Metrical<B>
where
    B: BackendDatabase + Send + Sync,
{
    pub fn new(storeful: Storeful<B>) -> Self {
        Self { storeful }
    }

    pub fn get_metrics(&self, primaries: &HashSet<Box<[u8]>>) -> Result<Vec<Metric>> {
        let results = self.storeful.backend.get_multi(primaries)?;
        let mut metrics = Vec::new();
        for result in results {
            metrics.push(bincode::deserialize(&result)?);
        }
        Ok(metrics)
    }
}

impl<B> ModelEndpoints<Metric, MetricQuery> for Metrical<B>
where
    B: BackendDatabase + Send + Sync,
{
    async fn post(&mut self, metric: Metric) -> Result<()> {
        let timestamp = format!("{:0>20}", metric.timestamp.timestamp_nanos_opt().unwrap());
        let primary = format!(
            "{}|{}|{}",
            metric.name,
            timestamp,
            metric.context.to_key_string()
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

        for context_value in metric.context.0 {
            self.storeful.backend.create_index(
                "context",
                &primary,
                &format!(
                    "context_value|{}:{}|{}",
                    context_value.key, context_value.value, &primary
                ),
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
            intersect(&mut primaries, name_primaries);
        }
        if query.timestamp_start.is_some() || query.timestamp_end.is_some() {
            let timestamp_primaries = self
                .storeful
                .backend
                .query_timestamp_index(query.timestamp_start, query.timestamp_end)?;
            intersect(&mut primaries, timestamp_primaries);
        }
        if let Some(context) = query.context {
            for context_value in context.0 {
                let context_value_key = format!(
                    "context_value|{}:{}|",
                    context_value.key, context_value.value
                );
                let context_value_primaries = self
                    .storeful
                    .backend
                    .query_index("context", &context_value_key)?;
                intersect(&mut primaries, context_value_primaries);
            }
        }
        self.get_metrics(&primaries)
    }
}

use std::sync::{Arc, Mutex};

use storage::Metrical;
use storeful::{prelude::*, sled::SledBackend, Args, Config, Storeful};

mod models;
mod query;
mod storage;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::default();

    let sled = SledBackend::open(
        args.db_path(),
        "metrics".into(),
        &["name", "timestamp", "labels"],
    )?;

    let storeful = Storeful::new(sled);
    let metrical = Metrical::new(storeful);

    let handler = Arc::new(Mutex::new(metrical));

    let config: Config = args.into();
    config.start(handler).await?;

    // Wait for all tasks to finish
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use chrono::Utc;
    use models::Metric;
    use query::MetricQuery;
    use rand::prelude::SliceRandom;
    use storage::Metrical;
    use storeful::{Label, Labels, ModelEndpoints, Storeful};

    #[tokio::test]
    async fn test() {
        let path = PathBuf::from("./test.db");
        let sled = SledBackend::open(&path, "metrics".into(), &["name", "timestamp", "labels"])
            .expect("Failed to open sled backend");
        let storeful = Storeful::new(sled);
        let mut metrical = Metrical::new(storeful);

        let start_time = Utc::now();

        let mut metrics = vec![];
        let metric_count = 1000;
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

        metrical.post_multi(metrics).await.unwrap();

        let written_time = Utc::now();

        // Do a query by all different fields
        let query = MetricQuery::empty()
            .with_name("memory_usage".into())
            .with_timestamp_start(0)
            .with_timestamp_end(metric_count / 2)
            .with_label(Label {
                key: "host".into(),
                value: "localhost".into(),
            })
            .with_label(Label {
                key: "region".into(),
                value: "us-west".into(),
            });

        let results = metrical.query(query).await.unwrap();

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
    }
}

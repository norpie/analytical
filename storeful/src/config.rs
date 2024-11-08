use std::sync::Arc;

use futures::future::join_all;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

use crate::{http, prelude::*, Args, ModelEndpoints, Query};

pub struct Config {
    pub host: String,
    pub port: u16,
    pub http: bool,
}

impl Config {
    pub async fn start<T, Q, M>(&self, handler: Arc<Mutex<M>>) -> Result<()>
    where
        T: Send + Sync + Serialize + DeserializeOwned + 'static,
        Q: Query,
        M: ModelEndpoints<T, Q> + Send + Sync + 'static,
    {
        let mut tasks = vec![];
        if self.http {
            tasks.push(http::start(handler.clone(), &self.host, self.port));
        }
        join_all(tasks).await.into_iter().collect::<Result<()>>()
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            host: args.host,
            port: args.port,
            http: args.http,
        }
    }
}

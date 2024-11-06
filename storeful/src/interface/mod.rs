use serde::{de::DeserializeOwned, Serialize};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use crate::prelude::*;

pub mod http;
// pub mod websocket;
// pub mod grpc;

pub trait Interface {
    fn start<T, Q, M>(
        &self,
        handler: Arc<Mutex<M>>,
        host: &str,
        post: u16,
    ) -> impl Future<Output = Result<()>>
    where
        T: Send + Sync + Serialize + DeserializeOwned + 'static,
        Q: Query,
        M: ModelEndpoints<T, Q> + Send + Sync + 'static;
}

pub trait ModelEndpoints<T, Q>
where
    T: Send + Sync + DeserializeOwned + Serialize + 'static,
    Q: Query,
{
    fn post(&mut self, input: T) -> impl Future<Output = Result<()>>;
    fn post_multi(&mut self, multi: Vec<T>) -> impl Future<Output = Result<()>>;
    fn query(&mut self, query: Q) -> impl Future<Output = Result<Vec<T>>>;
}

pub trait Query: Send + Sync + DeserializeOwned + Serialize + 'static {
    fn from_str(s: &str) -> Self;
    fn to_string(&self) -> String;
}

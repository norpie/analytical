use std::future::Future;
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;

pub mod http;
// pub mod websocket;
// pub mod grpc;

pub trait Interface<T> {
    fn start(&self, host: &str, post: u8) -> impl Future<Output = Result<()>>;
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

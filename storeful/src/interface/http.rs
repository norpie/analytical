use std::{future::Future, pin::Pin, sync::Arc, sync::Mutex};

use crate::{prelude::*, ModelEndpoints, Query};
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    server::conn::http1::Builder,
    service::Service,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpListener;

pub struct Http;

// impl Interface for Http {
pub async fn start<T, Q, M>(/*&self,*/ handler: Arc<Mutex<M>>, host: &str, port: u16) -> Result<()>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Q: Query,
    M: ModelEndpoints<T, Q> + Send + Sync + 'static,
{
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    let svc = Svc {
        handler,
        _t: std::marker::PhantomData,
        _q: std::marker::PhantomData,
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Builder::new().serve_connection(io, svc_clone).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
// }

#[derive(Debug)]
struct Svc<M, T, Q>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Q: Query,
    M: ModelEndpoints<T, Q> + Send + Sync + 'static,
{
    handler: Arc<Mutex<M>>,
    _t: std::marker::PhantomData<T>,
    _q: std::marker::PhantomData<Q>,
}

impl<M, Q, T> Clone for Svc<M, T, Q>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Q: Query,
    M: ModelEndpoints<T, Q> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _t: std::marker::PhantomData,
            _q: std::marker::PhantomData,
        }
    }
}

impl<M, T, Q> Service<Request<IncomingBody>> for Svc<M, T, Q>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Q: Query,
    M: ModelEndpoints<T, Q> + Send + Sync + 'static,
{
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        let _handler = self.handler.lock().unwrap();
        fn mk_response(s: String) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        if req.uri().path() != "/favicon.ico" {
            mk_response("favicon".into()).unwrap();
        }

        let res = match req.uri().path() {
            "/" => mk_response("index".into()),
            _ => mk_response("oh no! not found".into()),
        };

        Box::pin(async { res })
    }
}

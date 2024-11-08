use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, PoisonError},
};

use crate::{prelude::*, ModelEndpoints, Query, Storeful};
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    server::conn::http1::Builder,
    service::Service,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};

pub struct Http;

// impl Interface for Http {
pub async fn start<T, Q, M>(
    /*&self,*/ handler: Arc<Mutex<M>>,
    host: &str,
    port: u16,
) -> Result<()>
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
        tokio::task::spawn(async {
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
        fn ok(content: String) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(format!(
                    "{{\"result\": {}}}",
                    content
                ))))
                .unwrap())
        }

        fn error(error: String) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(format!(
                    "{{\"error\": \"{}\"}}",
                    error
                ))))
                .unwrap())
        }

        let handler = self.handler.clone();

        Box::pin(async move {
            let result = request(handler, req).await;
            match result {
                Ok(json) => ok(json),
                Err(e) => error(e.0),
            }
        })
    }
}

async fn request<T, Q, M>(
    handler: Arc<Mutex<M>>,
    req: Request<IncomingBody>,
) -> std::result::Result<String, HttpError>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Q: Query,
    M: ModelEndpoints<T, Q> + Send + Sync + 'static,
{
    let mut handler = handler.lock().await;
    if req.method() != hyper::Method::POST {
        return Err(HttpError("method not allowed".into()));
    }
    let uri = req.uri().clone();
    let path = uri.path();
    let bytes = req.collect().await?;
    match path {
        "/query" => {
            let query: Q = serde_json::from_slice(&bytes.to_bytes())?;
            let result = handler.query(query).await?;
            Ok(serde_json::to_string(&result)?)
        }
        "/post" => {
            let model: T = serde_json::from_slice(&bytes.to_bytes())?;
            handler.post(model).await?;
            Ok("ok".to_string())
        }
        "/post_multi" => {
            let models: Vec<T> = serde_json::from_slice(&bytes.to_bytes())?;
            handler.post_multi(models).await?;
            Ok("ok".to_string())
        }
        _ => Err(HttpError("not found".into())),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError(String);

impl From<StorefulError> for HttpError {
    fn from(e: StorefulError) -> Self {
        Self(format!("storeful error: {}", e))
    }
}

impl From<hyper::Error> for HttpError {
    fn from(e: hyper::Error) -> Self {
        Self(format!("hyper error: {}", e))
    }
}

impl<T> From<PoisonError<T>> for HttpError {
    fn from(_: PoisonError<T>) -> Self {
        Self("lock poisoned".to_string())
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(e: serde_json::Error) -> Self {
        Self(format!("json error: {}", e))
    }
}

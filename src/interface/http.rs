use anyhow::Error;
use serde::Deserialize;
use tide::{Request, Response};

use crate::{Metric, Metrical};

pub async fn serve() -> Result<(), Error> {
    let hostname = "localhost";
    let port = 4340;

    let addr = format!("{}:{}", hostname, port);

    let mut app = tide::new();
    app.at("/ping").get(ping);

    app.at("/metrics").post(add_metric);
    app.at("/metrics").get(get_metrics);

    println!("Listening on http://{}", addr);
    app.listen(addr).await?;
    Ok(())
}

// Return "pong" for a GET request to /ping
async fn ping(_req: Request<()>) -> tide::Result {
    Ok("pong".into())
}

async fn add_metric(mut req: Request<()>) -> tide::Result {
    let metric: Metric = req.body_json().await?;
    let mut metrical = Metrical::get_instance().write().await;
    metrical.add_metric(metric)?;
    Ok(Response::new(200))
}

#[derive(Debug, Deserialize)]
struct MetricQuery {
    name: String,
    key: String,
}

async fn get_metrics(req: Request<()>) -> tide::Result {
    let query: MetricQuery = req.query()?;
    let mut metrical = Metrical::get_instance().write().await;
    let metrics = metrical.get_metrics(&query.name, &query.key)?;
    let mut response = Response::new(200);
    let json = serde_json::to_string(&metrics)?;
    response.set_body(json);
    response.set_content_type("application/json");
    Ok(response)
}

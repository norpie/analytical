use anyhow::Error;
use tide::Request;

pub async fn serve() -> Result<(), Error> {
    let hostname = "localhost";
    let port = 4340;

    let addr = format!("{}:{}", hostname, port);

    let mut app = tide::new();
    app.at("/ping").get(ping);

    println!("Listening on http://{}", addr);
    app.listen(addr).await?;
    Ok(())
}

// Return "pong" for a GET request to /ping
async fn ping(_req: Request<()>) -> tide::Result {
    Ok("pong".into())
}

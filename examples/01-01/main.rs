use anyhow::Result;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}

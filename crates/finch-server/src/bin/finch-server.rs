use std::net::SocketAddr;

use finch_server::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(finch_server::app().await?.into_make_service())
        .await
        .unwrap();

    Ok(())
}

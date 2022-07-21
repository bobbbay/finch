//! A general health check.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use finch_server::{app, error::Result};
use tower::ServiceExt;

#[tokio::test]
async fn health() -> Result<()> {
    let app = app().await?;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    insta::assert_debug_snapshot!(body);

    Ok(())
}

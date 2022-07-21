pub mod authentication;
pub mod database;
pub mod error;
pub mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use database::construct_database;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::authentication::{construct_authority, construct_authorizer};
use crate::error::Result;

pub async fn app() -> Result<Router> {
    let database = construct_database().await?;

    let authority = construct_authority().await?;
    let authorizer = construct_authorizer().await?;

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(database))
        .layer(authorizer.jwt_layer(authority))
        .into_inner();

    let router = Router::new()
        .route("/create_user", post(handlers::create_user))
        .layer(middleware_stack);

    Ok(router)
}

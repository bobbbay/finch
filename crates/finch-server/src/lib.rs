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

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(database))
        .into_inner();

    let public_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/example_public", get(handlers::example_public));

    let private_routes = Router::new()
        .route("/example_private", get(handlers::example_private))
        .route("/create_user", post(handlers::create_user))
        .layer(authorizer.jwt_layer(authority));

    let app = public_routes.merge(private_routes).layer(middleware);

    Ok(app)
}

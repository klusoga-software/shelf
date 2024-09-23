use crate::error::Error;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, Layer};

mod controller;

mod error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let router = Router::new()
        .nest("/health", controller::health_controller::get_router())
        .layer(TraceLayer::new_for_http().make_span_with(|request: &axum::http::request::Request<_>| {
            tracing::info_span!("HTTP Request", method = %request.method(), uri = %request.uri())
        }).on_request(|request: &axum::http::request::Request<_>, span: &tracing::Span|{
            tracing::info!("Request received")
        }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6300").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

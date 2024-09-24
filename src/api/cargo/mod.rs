use crate::api::cargo::auth::me;
use crate::api::cargo::index::index;
use axum::routing::get;
use axum::Router;

pub mod auth;
pub mod index;

pub mod models;

pub fn get_router() -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/index", get(index))
}

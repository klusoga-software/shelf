use crate::api::cargo::auth::me;
use crate::api::cargo::crates::new;
use crate::api::cargo::index::index;
use axum::routing::{get, put};
use axum::Router;

pub mod auth;
pub mod index;

pub mod models;

pub mod crates;

pub fn get_router() -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/api/v1/crates/new", put(new))
        .nest("/index", index())
}

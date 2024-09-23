use crate::api::cargo::auth::me;
use axum::routing::get;
use axum::Router;

pub mod auth;

pub fn get_router() -> Router {
    Router::new().route("/me", get(me))
}

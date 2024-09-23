use axum::Router;

pub mod cargo;
pub mod models;

pub fn get_router() -> Router {
    Router::new().nest("/cargo", cargo::get_router())
}

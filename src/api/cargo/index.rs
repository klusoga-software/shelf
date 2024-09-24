use axum::Router;
use tower_http::services::ServeDir;

pub fn index() -> Router {
    Router::new().nest_service("/", ServeDir::new("assets"))
}

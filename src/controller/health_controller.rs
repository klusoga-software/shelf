use axum::routing::get;
use axum::Router;

pub fn get_router() -> Router {
    Router::new().route("/", get(get_health))
}

pub async fn get_health() -> &'static str {
    "OK"
}

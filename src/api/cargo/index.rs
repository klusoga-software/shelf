use crate::api::cargo::models::Index;
use crate::api::models::Error;
use axum::Json;

pub async fn index() -> Result<Json<Index>, Json<Error>> {
    let dl = format!(
        "{}/crates",
        std::env::var("BASE_URL").unwrap_or("http://localhost:6300".to_string())
    );

    let api = std::env::var("BASE_URL").unwrap_or("http://localhost:6300".to_string());

    Ok(Json::from(Index { dl, api: Some(api) }))
}

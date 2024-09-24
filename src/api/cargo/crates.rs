use crate::api::models::Error;
use axum::Json;

pub async fn new() -> Result<Json<Error>, Json<Error>> {
    Ok(Json(Error {
        detail: "Some error message".to_string(),
    }))
}

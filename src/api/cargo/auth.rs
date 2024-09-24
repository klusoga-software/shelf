use std::env::VarError;

pub async fn me() -> String {
    std::env::var("AUTH_URL").unwrap_or("http://localhost:6300".to_string())
}

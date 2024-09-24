
pub async fn me() -> String {
    format!(
        "{}/cargo/auth",
        std::env::var("BASE_URL").unwrap_or("http://localhost:6300".to_string())
    )
}

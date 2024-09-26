use actix_web::{get, HttpResponse, Responder};

#[get("/me")]
pub async fn me() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "{}/cargo/auth",
        std::env::var("BASE_URL").unwrap_or("http://localhost:6300".to_string())
    ))
}

use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

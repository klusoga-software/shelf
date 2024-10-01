use crate::configuration::Configuration;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/configuration")]
pub async fn get_configuration(config: web::Data<Configuration>) -> impl Responder {
    HttpResponse::Ok().json(&config.ui)
}

use crate::api::cargo::get_cargo_scope;
use crate::controller::health_controller::get_health;
use crate::error::Error;
use actix_web::{App, HttpServer};

mod controller;

mod error;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_health).service(get_cargo_scope()))
        .bind(("0.0.0.0", 6300))?
        .run()
        .await
}

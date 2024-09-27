use crate::api::cargo::get_cargo_scope;
use crate::controller::health_controller::get_health;
use crate::repository::cargo_repository::CargoRepository;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Logger, Next};
use actix_web::{web, App, Error, HttpServer};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

mod controller;

mod error;

mod api;

mod repository;

async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    next.call(req).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let connection_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_url.as_str())
        .await
        .expect("Failed to connect to database");

    let cargo_repository = CargoRepository::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(from_fn(auth))
            .app_data(web::Data::new(cargo_repository.clone()))
            .service(get_health)
            .service(get_cargo_scope())
    })
    .bind(("0.0.0.0", 6300))?
    .run()
    .await
}

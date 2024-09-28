use crate::api::cargo::get_cargo_scope;
use crate::api::health_controller::get_health;
use crate::api::repo_controller::repo_controller;
use crate::repository::cargo_repository::CargoRepository;
use crate::storage::local::LocalStorage;
use crate::storage::s3::S3Storage;
use crate::storage::Storage;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Logger, Next};
use actix_web::{web, App, Error, HttpServer};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

mod error;

mod api;

mod repository;

mod storage;

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
        let storage: Box<dyn Storage> = match std::env::var("STORAGE_TYPE")
            .unwrap_or("LOCAL".to_string())
            .as_str()
        {
            "LOCAL" => Box::from(LocalStorage {}),

            "S3" => Box::from(S3Storage {}),

            _ => panic!("None storage type matches. Please specify one of [LOCAL, S3]"),
        };

        App::new()
            .wrap(Logger::default())
            .wrap(from_fn(auth))
            .app_data(web::Data::new(cargo_repository.clone()))
            .app_data(web::Data::new(storage))
            .service(get_health)
            .service(repo_controller())
            .service(get_cargo_scope())
    })
    .bind(("0.0.0.0", 6300))?
    .run()
    .await
}
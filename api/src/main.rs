use crate::api::api_scope;
use crate::api::cargo::get_cargo_scope;
use crate::configuration::Configuration;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::role_repository::RoleRepository;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use crate::storage::local::LocalStorage;
use crate::storage::s3::S3Storage;
use crate::storage::Storage;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use std::{env, fs};

mod api;
mod auth;
mod configuration;
mod error;
mod jwt;
mod repository;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let connection_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@localhost/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_url.as_str())
        .await
        .expect("Failed to connect to database");

    let cargo_repository = CargoRepository::new(pool.clone());
    let service_account_repository = ServiceAccountsRepository::new(pool.clone());
    let role_repository = RoleRepository::new(pool.clone());

    let binding = env::var("HTTP_BINDING").unwrap_or("0.0.0.0:6300".to_string());

    let configuration = load_configuration();

    HttpServer::new(move || {
        let storage: Box<dyn Storage> = match env::var("STORAGE_TYPE")
            .unwrap_or("LOCAL".to_string())
            .as_str()
        {
            "LOCAL" => Box::from(LocalStorage {}),

            "S3" => {
                let s3_configuration = configuration
                    .s3
                    .as_ref()
                    .expect("Unable to load s3 configuration");

                let bucket = match Bucket::new(
                    &s3_configuration.bucket,
                    Region::Custom {
                        region: s3_configuration.region.clone(),
                        endpoint: s3_configuration.host.clone(),
                    },
                    Credentials::new(
                        Some(&s3_configuration.access_key),
                        Some(&s3_configuration.secret_key),
                        None,
                        None,
                        None,
                    )
                    .expect("Unable to parse s3 credentials"),
                ) {
                    Ok(bucket) => bucket,
                    Err(err) => panic!("Unable to access bucket {}", err),
                };

                Box::from(S3Storage::new(bucket))
            }

            _ => panic!("None storage type matches. Please specify one of [LOCAL, S3]"),
        };

        let ui_directory = env::var("UI_DIRECTORY").unwrap_or("./dist".to_string());

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .wrap(Logger::default())
            .app_data(Data::new(cargo_repository.clone()))
            .app_data(Data::new(service_account_repository.clone()))
            .app_data(Data::new(role_repository.clone()))
            .app_data(Data::new(storage))
            .app_data(Data::new(load_configuration()))
            .service(get_cargo_scope())
            .service(api_scope())
            .service(
                actix_files::Files::new(
                    "/assets",
                    Path::new(&ui_directory)
                        .join("assets")
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
                .show_files_listing(),
            )
            .service(actix_files::Files::new("/{all}*", ui_directory).index_file("index.html"))
    })
    .bind(binding)?
    .run()
    .await
}

fn load_configuration() -> Configuration {
    let config_path = env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());

    let config_file = match fs::read_to_string(&config_path) {
        Ok(file) => file,
        Err(err) => panic!("Failed to read config file: {}", err),
    };

    match toml::from_str::<Configuration>(&config_file) {
        Ok(config) => config,
        Err(err) => panic!("Failed to parse config file: {}", err),
    }
}

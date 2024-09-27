use crate::api::cargo::models::IndexConfig;
use crate::api::models::{Error, ErrorResponse};
use crate::repository::cargo_repository::CargoRepository;
use actix_web::http::header::ContentType;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
use std::{fs::File, io::Read};

#[get("/{name}/index/config.json")]
pub async fn config(name: Path<String>, pool: web::Data<CargoRepository>) -> impl Responder {
    let repo = match pool.get_repo_by_name(name.as_str()).await {
        Ok(repo) => repo,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let config_doc = match pool.get_config_by_repo(&repo.id).await {
        Ok(config) => config,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let index_config = IndexConfig {
        dl: config_doc.dl,

        api: config_doc.api,
        auth_required: !repo.public,
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&index_config).unwrap())
}

#[get("/index/{name:.*}")]
pub async fn index_files(name: web::Path<String>) -> impl Responder {
    let mut file = match File::open(format!("assets/{}", name)) {
        Ok(file) => file,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                errors: vec![Error {
                    detail: "The requested crate was not found".to_string(),
                }],
            })
        }
    };

    let mut config_content = String::new();

    file.read_to_string(&mut config_content).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(config_content)
}

use crate::api::cargo::models::IndexConfig;
use crate::repository::cargo_repository::CargoRepository;
use actix_web::http::header::ContentType;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/{name}/index/config.json")]
pub async fn config(name: Path<String>, pool: web::Data<CargoRepository>) -> impl Responder {
    let repo = match pool.get_repo_by_name(name.as_str()).await {
        Ok(repo) => repo,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let config_doc = match pool.get_config_by_repo(&repo.id.unwrap()).await {
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

#[get("/{name}/index/{crate_name:.*}")]
pub async fn index_files(
    path: Path<(String, String)>,
    state: web::Data<CargoRepository>,
) -> impl Responder {
    let (name, crate_name) = path.into_inner();

    let crate_name = crate_name.rsplit('/').next().unwrap_or(&crate_name);

    let repo = match state.get_repo_by_name(name.as_str()).await {
        Ok(repo) => repo,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let crate_index = match state
        .get_index_by_name_and_id(crate_name, &repo.id.unwrap())
        .await
    {
        Ok(index) => index,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut index_response = String::new();

    for index in crate_index {
        let index_json = match serde_json::to_string(&index.index) {
            Ok(json) => json,
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        };

        index_response.push_str(&index_json);
        index_response.push('\n');
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(index_response)
}

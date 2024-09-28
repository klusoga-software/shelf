use crate::api::models::CreateRepoRequest;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::models::{Config, Repo};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use std::env;

pub fn repo_controller() -> actix_web::Scope {
    web::scope("/repo")
        .service(create_repo)
        .service(get_repos)
        .service(delete_repo)
}

#[post("")]
pub async fn create_repo(
    state: web::Data<CargoRepository>,
    repo: web::Json<CreateRepoRequest>,
) -> impl Responder {
    let base_url = env::var("BASE_URL").unwrap_or("http://localhost:6300".to_string());

    let repo_id = match state
        .create_repo(Repo {
            id: None,
            name: repo.name.clone(),
            public: false,
            repo_type: repo.repo_type.clone(),
        })
        .await
    {
        Ok(id) => id,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    match state
        .create_config(Config {
            id: None,
            repo_id,
            dl: format!("{}/cargo/{}/crates", &base_url, &repo.name),
            api: format!("{}/cargo/{}", &base_url, &repo.name),
        })
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("")]
pub async fn get_repos(state: web::Data<CargoRepository>) -> impl Responder {
    match state.get_repos().await {
        Ok(repos) => HttpResponse::Ok().json(repos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_repo(state: web::Data<CargoRepository>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    match state.delete_config(&id).await {
        Ok(_) => {}
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    match state.delete_repo(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

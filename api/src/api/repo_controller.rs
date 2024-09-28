use crate::api::models::CreateRepoRequest;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::models::Repo;
use actix_web::web::delete;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub fn repo_controller() -> actix_web::Scope {
    web::scope("/repo")
        .service(create_repo)
        .service(get_repos)
        .service(delete_repo)
}

#[post("/")]
pub async fn create_repo(
    state: web::Data<CargoRepository>,
    repo: web::Json<CreateRepoRequest>,
) -> impl Responder {
    match state
        .create_repo(Repo {
            id: None,
            name: repo.name.clone(),
            public: false,
            repo_type: repo.repo_type.clone(),
        })
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/")]
pub async fn get_repos(state: web::Data<CargoRepository>) -> impl Responder {
    match state.get_repos().await {
        Ok(repos) => HttpResponse::Ok().json(repos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_repo(state: web::Data<CargoRepository>, id: web::Path<i32>) -> impl Responder {
    match state.delete_repo(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

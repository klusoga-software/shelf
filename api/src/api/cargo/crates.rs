use crate::api::cargo::models::{CrateIndex, Metadata};
use crate::auth::check_auth;
use crate::error::{AuthError, Error};
use crate::log_error_and_responde;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::models::Crate;
use crate::storage::Storage;
use actix_files::NamedFile;
use actix_web::web::{Buf, Bytes};
use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::types::Json;
use std::io::Read;
use std::io::{self, Write};

#[put("/{name}/api/v1/crates/new")]
pub async fn upload(
    name: web::Path<String>,
    body: Bytes,
    state: web::Data<CargoRepository>,
    storage_state: web::Data<Box<dyn Storage>>,
    req: HttpRequest,
) -> impl Responder {
    match check_auth(req, "W".to_string()).await {
        Ok(_) => {}
        Err(err) => match err {
            AuthError::Unauthorized(message) => return HttpResponse::Unauthorized().body(message),
            AuthError::ActixDataMissing(message) => {
                return HttpResponse::InternalServerError().body(message)
            }
            AuthError::RepositoryNotFound(repo) => return HttpResponse::NotFound().body(repo),
        },
    }

    let (crate_index, crate_file) = match parse_crate(body.reader()) {
        Ok(res) => res,
        Err(err) => return log_error_and_responde!(err),
    };

    let repo = match state.get_repo_by_name(name.into_inner().as_str()).await {
        Ok(repo) => repo,
        Err(err) => return log_error_and_responde!(err),
    };

    let crate_name = crate_index.name.clone();
    let crate_path = format!("crates/{}_{}.crate", crate_name, crate_index.vers.clone());

    match state
        .add_index(Crate {
            name: crate_name.clone(),
            version: crate_index.vers.clone(),
            index: Json(crate_index),
            path: crate_path.clone(),
            repo_id: repo.id.unwrap(),
            id: None,
        })
        .await
    {
        Ok(_) => {}
        Err(err) => return log_error_and_responde!(err),
    };

    match storage_state.save(crate_path, crate_file) {
        Ok(_) => {}
        Err(err) => return log_error_and_responde!(err),
    };

    HttpResponse::Ok().json(json!({}))
}

#[get("/{name}/crates/{crate_name}/{version}/download")]
pub async fn download(
    path: web::Path<(String, String, String)>,
    state: web::Data<CargoRepository>,
) -> actix_web::Result<NamedFile> {
    let (name, crate_name, version) = path.into_inner();

    let repo = match state.get_repo_by_name(&name).await {
        Ok(repo) => repo,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    };

    let crate_index = match state
        .get_index_by_name_id_and_version(&crate_name, &version, &repo.id.unwrap())
        .await
    {
        Ok(index) => index,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    };

    let crate_index = match crate_index {
        None => return Err(actix_web::error::ErrorNotFound("crate not found")),
        Some(index) => index,
    };

    let file = match NamedFile::open(crate_index.path) {
        Ok(file) => file,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    };

    Ok(file)
}

fn parse_crate<R: Read>(mut reader: R) -> Result<(CrateIndex, Vec<u8>), Error> {
    fn read_u32_le<R: Read>(reader: &mut R) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    let json_len = read_u32_le(&mut reader)? as usize;

    let mut json_buffer = vec![0u8; json_len];

    reader.read_exact(&mut json_buffer)?;

    let metadata: Metadata = match serde_json::from_slice(&json_buffer) {
        Ok(meta) => meta,
        Err(err) => {
            return Err(Error::CrateParse(format!(
                "Failed to parse metadata: {}",
                err
            )))
        }
    };

    let crate_len = read_u32_le(&mut reader)? as usize;

    let mut crate_buffer = vec![0u8; crate_len];

    reader.read_exact(&mut crate_buffer)?;

    let mut hasher = Sha256::new();
    hasher.write_all(&crate_buffer)?;
    let checksum: String = format!("{:x}", hasher.finalize());

    let crate_index = CrateIndex::new_from_metadata(&metadata, checksum);

    Ok((crate_index, crate_buffer))
}

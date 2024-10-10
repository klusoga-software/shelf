use crate::error::Error;
use actix_files::NamedFile;
use async_trait::async_trait;

pub mod local;

pub mod s3;

#[async_trait]
pub trait Storage {
    async fn save(&self, path: String, data: Vec<u8>) -> Result<(), Error>;
    async fn download(&self, path: String) -> Result<NamedFile, Error>;
    async fn remove(&self, path: String) -> Result<(), Error>;
}

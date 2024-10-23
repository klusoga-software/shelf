use crate::error::Error;
use crate::storage::Storage;
use actix_files::NamedFile;
use async_trait::async_trait;

#[derive(Clone)]
pub struct LocalStorage {}

impl LocalStorage {
    pub fn new() -> Self {
        if let Ok(false) = std::fs::exists("crates") {
            std::fs::create_dir("crates").expect("Unable to create directory \"crates\"");
        }

        Self {}
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn save(&self, path: String, data: Vec<u8>) -> Result<(), Error> {
        let mut file = std::fs::File::create(path)?;

        std::io::Write::write_all(&mut file, &data)?;
        Ok(())
    }

    async fn download(&self, path: String) -> Result<NamedFile, Error> {
        NamedFile::open(path).map_err(Error::IO)
    }

    async fn remove(&self, path: String) -> Result<(), Error> {
        std::fs::remove_file(path).map_err(Error::IO)
    }
}

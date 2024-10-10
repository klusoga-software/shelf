use crate::error::Error;
use crate::storage::Storage;
use actix_files::NamedFile;
use async_trait::async_trait;
use s3::Bucket;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
pub struct S3Storage {
    bucket: Box<Bucket>,
}

impl S3Storage {
    pub fn new(bucket: Box<Bucket>) -> Self {
        S3Storage { bucket }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn save(&self, path: String, data: Vec<u8>) -> Result<(), Error> {
        match self.bucket.put_object(path, data.as_slice()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::S3(err)),
        }
    }

    async fn download(&self, path: String) -> Result<NamedFile, Error> {
        match self.bucket.get_object(path).await {
            Ok(data) => {
                let file_name = uuid::Uuid::new_v4().to_string();
                let mut file = File::create_new(&file_name).map_err(Error::IO)?;

                file.write_all(data.as_slice()).map_err(Error::IO)?;

                match NamedFile::from_file(file, file_name) {
                    Ok(file) => {
                        std::fs::remove_file(file.path()).map_err(Error::IO)?;
                        Ok(file)
                    }
                    Err(err) => Err(Error::IO(err)),
                }
            }
            Err(err) => Err(Error::S3(err)),
        }
    }

    async fn remove(&self, path: String) -> Result<(), Error> {
        match self.bucket.delete_object(path).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::S3(err)),
        }
    }
}

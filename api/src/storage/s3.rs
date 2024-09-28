use crate::error::Error;
use crate::storage::Storage;

#[derive(Clone)]
pub struct S3Storage {}

impl Storage for S3Storage {
    fn save(&self, path: String, data: Vec<u8>) -> Result<(), Error> {
        todo!()
    }
}

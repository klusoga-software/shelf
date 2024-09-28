use crate::error::Error;

pub mod local;

pub mod s3;

pub trait Storage {
    fn save(&self, path: String, data: Vec<u8>) -> Result<(), Error>;
}

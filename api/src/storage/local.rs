use crate::error;
use crate::storage::Storage;

#[derive(Clone)]
pub struct LocalStorage {}

impl Storage for LocalStorage {
    fn save(&self, path: String, data: Vec<u8>) -> Result<(), error::Error> {
        let mut file = std::fs::File::create(path)?;

        std::io::Write::write_all(&mut file, &data)?;
        Ok(())
    }
}

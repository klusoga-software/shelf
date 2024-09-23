use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

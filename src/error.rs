use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    CrateParse(String),
    JsonParse(serde_json::Error),
    VersionExists(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParse(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => {
                write!(f, "{}", err)
            }
            Error::CrateParse(err) => {
                write!(f, "Crate Parse Error: {}", err)
            }
            Error::JsonParse(err) => {
                write!(f, "Parse Json failed: {}", err)
            }
            Error::VersionExists(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

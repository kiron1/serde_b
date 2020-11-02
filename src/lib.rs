use std::io;

pub mod ser;

#[derive(Debug)]
pub enum Error {
    NotSerializeable,
    Io(io::Error),
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSerializeable => write!(f, "not serializeable"),
            Self::Io(ref cause) => write!(f, "IO error: {}", cause),
        }
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

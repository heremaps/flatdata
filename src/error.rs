use std::error;
use std::fmt;
use std::str::Utf8Error;
use std::io;

#[derive(Debug)]
pub enum ResourceStorageError {
    Io(io::Error, String),
    MissingSchema(String),
    UnexpectedDataSize,
    Utf8Error(Utf8Error),
    WrongSignature { resource_name: String, diff: String },
}

impl ResourceStorageError {
    pub fn from_io_error(err: io::Error, resource_name: String) -> Self {
        ResourceStorageError::Io(err, resource_name)
    }
}

impl fmt::Display for ResourceStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ResourceStorageError {
    fn description(&self) -> &str {
        match *self {
            ResourceStorageError::Io(_, _) => "resource io error",
            ResourceStorageError::MissingSchema(_) => "schema of resource is missing",
            ResourceStorageError::UnexpectedDataSize => "resource has unexpected size",
            ResourceStorageError::Utf8Error(_) => "utf8 error in schema",
            ResourceStorageError::WrongSignature { .. } => "schema is not matching expected schema",
        }
    }
}

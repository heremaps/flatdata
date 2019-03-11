use std::error;
use std::fmt;
use std::io;
use std::str::Utf8Error;

/// Error indicating failures when reading and writing data from/to a
/// [`Storage`].
///
/// [`Storage`]: trait.Storage.html
#[derive(Debug)]
pub enum ResourceStorageError {
    /// Wrapper of [`io::Error`] with resource name for which the error
    /// occurred.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    Io(io::Error, String),
    /// Wrapper of [`Utf8Error`].
    ///
    /// [`Utf8Error`]: https://doc.rust-lang.org/std/str/struct.Utf8Error.html
    Utf8Error(Utf8Error),
    /// Indicates that schema for the resource with stored name is missing in
    /// resource storage.
    MissingSchema(String),
    /// Indicates that the schema stored in resource storage differs from the
    /// expected schema.
    WrongSignature {
        /// Resource name for which the error occurred.
        resource_name: String,
        /// Diff from the stored schema to the expected schema.
        diff: String,
    },
    /// Indicates that the size of the data does not fit to the serialized
    /// control size.
    ///
    /// When data is serialized to resource storage, a control header is
    /// written which, in particular, contains the final size of the whole
    /// resource.
    UnexpectedDataSize,
}

impl ResourceStorageError {
    /// Wraps an [`io::Error`] with additional resource name.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
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

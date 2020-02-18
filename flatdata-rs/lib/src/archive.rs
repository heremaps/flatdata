//! This module contains archive traits that are implemented by the generated code
//! for each flatdata archive.

use crate::{error::ResourceStorageError, storage::ResourceStorage};

use std::{fmt::Debug, sync::Arc};

#[doc(hidden)]
pub use std::marker;

/// A flatdata archive representing serialized data.
///
/// Each archive in generated code implements this trait.
pub trait Archive: Debug + Clone + Send {
    /// Name of the archive.
    const NAME: &'static str;
    /// Schema of the archive.
    ///
    /// Used for verifying the integrity of the archive when opening.
    const SCHEMA: &'static str;

    /// Opens the archive with name `NAME` and schema `SCHEMA` in the given
    /// storage for reading.
    ///
    /// When opening the archive, the schema of the archive and the schema
    /// stored in the storage are compared as strings. If there is a
    /// difference, an Error [`ResourceStorageError::WrongSignature`](enum.
    /// ResourceStorageError.html) is returned containing a detailed diff
    /// of both schemata.
    ///
    /// All resources are in the archive are also opened and their schemata are
    /// verified. If any non-optional resource is missing or has a wrong
    /// signature (unexpected schema), the operation will fail. Therefore,
    /// it is not possible to open partially written archive.
    fn open(storage: Arc<dyn ResourceStorage>) -> Result<Self, ResourceStorageError>;
}

/// A flatdata archive builder for serializing data.
///
/// For each archive in generated code there is a corresponding archive builder
/// which implements this trait.
pub trait ArchiveBuilder: Clone + Send {
    /// Name of the archive associated with this archive builder.
    const NAME: &'static str;
    /// Schema of the archive associated with this archive builder.
    ///
    /// Used only for debug and inspection purposes.
    const SCHEMA: &'static str;

    /// Creates an archive with name `NAME` and schema `SCHEMA` in the given
    /// storage for writing.
    ///
    /// If the archive is successfully created, the storage will contain the
    /// archive and archives schema. Archive's resources need to be written
    /// separately by using the corresponding generated methods:
    ///
    /// * `set_struct`
    /// * `set_vector`
    /// * `start_vector`/`close`
    /// * `start_multivector`/`close`.
    ///
    /// For more information about how to write resources, cf. the
    /// [coappearances] example.
    ///
    /// [coappearances]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs#L159
    fn new(storage: Arc<dyn ResourceStorage>) -> Result<Self, ResourceStorageError>;
}

#[cfg(test)]
mod tests {
    use super::{Archive, ArchiveBuilder};
    use crate::test::{XBuilder, X};
    use crate::MemoryResourceStorage;

    use std::thread;

    #[test]
    fn test_send_builder() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let builder = XBuilder::new(storage.clone()).unwrap();

        let th = thread::spawn(move || {
            let mut data = builder.start_data().unwrap();
            data.grow().unwrap();
            data.close().unwrap();
        });
        th.join().unwrap();

        let archive = X::open(storage).unwrap();
        assert_eq!(archive.data().len(), 1);
    }

    #[test]
    fn test_send_archive() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let builder = XBuilder::new(storage.clone()).unwrap();
        let mut data = builder.start_data().unwrap();
        data.grow().unwrap();
        data.close().unwrap();

        let archive = X::open(storage).unwrap();
        let th = thread::spawn(move || {
            assert_eq!(archive.data().len(), 1);
        });
        th.join().unwrap();
    }
}

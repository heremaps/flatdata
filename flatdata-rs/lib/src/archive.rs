//! This module contains traits and macros that are used by generated code to
//! define flatdata's structs, archives and resources.
//!
//! # Archive
//!
//! A flatdata archive is introduced by `define_archive`. It defines two types
//! `ArchiveName` and `ArchiveNameBuilder` for reading resp. writing data.

use crate::{error::ResourceStorageError, storage::ResourceStorage};

use std::{fmt::Debug, rc::Rc};

#[doc(hidden)]
pub use std::marker;

/// A flatdata archive representing serialized data.
///
/// Each archive in generated code implements this trait.
pub trait Archive: Debug + Clone {
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
    fn open(storage: Rc<dyn ResourceStorage>) -> Result<Self, ResourceStorageError>;
}

/// A flatdata archive builder for serializing data.
///
/// For each archive in generated code there is a corresponding archive builder
/// which implements this trait.
pub trait ArchiveBuilder: Clone {
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
    /// * `start_vector`/`finish_vector`
    /// * `start_multivector`/`finish_multivector`.
    ///
    /// For more information about how to write resources, cf. the
    /// [coappearances] example.
    ///
    /// [coappearances]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs#L159
    fn new(storage: Rc<dyn ResourceStorage>) -> Result<Self, ResourceStorageError>;
}

/// Macro used by generator to define a flatdata archive and corresponding
/// archive builder.
#[doc(hidden)]
#[macro_export]
macro_rules! define_archive {
    // prelude of internal helpers (see https://danielkeep.github.io/tlborm/book/pat-internal-rules.html)

    // static if
    (@if, true, $true_block:block, $false_block:block) => {
        $true_block
    };
    (@if, false, $true_block:block, $false_block:block) => {
        $false_block
    };

    // define member types
    (@members, multivector(true $($args:tt)*)) => {
        Option<($crate::MemoryDescriptor, $crate::MemoryDescriptor)>
    };
    (@members, multivector(false $($args:tt)*)) => {
        ($crate::MemoryDescriptor, $crate::MemoryDescriptor)
    };
    (@members, archive(true, $schema:expr, $type:path, $builder_type:path)) => {
        Option<$type>
    };
    (@members, archive(false, $schema:expr, $type:path, $builder_type:path)) => {
        $type
    };
    (@members, $type:ident(true $($args:tt)*)) => {
        Option<$crate::MemoryDescriptor>
    };
    (@members, $type:ident(false $($args:tt)*)) => {
        $crate::MemoryDescriptor
    };

    // check resources
    (@check, $res:expr, false) => {
        $res?
    };
    (@check, $res:expr, true) => {
        $res.ok()
    };

    // read resources
    (@read, $storage:ident, raw_data($name:ident, $optional:ident, $schema:expr, $setter:ident)) => {{
        define_archive!(@check,
            Self::read_resource(&*$storage, stringify!($name), $schema),
            $optional
        )
    }};
    (@read, $storage:ident, struct($name:ident, $optional:ident, $schema:expr, $setter:ident, $type:path)) => {{
        define_archive!(@check,
            Self::read_resource(&*$storage, stringify!($name), $schema),
            $optional
        )
    }};
    (@read, $storage:ident, vector($name:ident, $optional:ident, $schema:expr, $setter:ident, $starter:ident, $type:path)) => {{
        define_archive!(@check,
            Self::read_resource(&*$storage, stringify!($name), $schema),
            $optional
        )
    }};
    (@read, $storage:ident, multivector($name:ident, $optional:ident, $schema:expr, $starter:ident, $variadic_type:path, $index:ident, $index_type:path)) => {{
        let index_schema = &format!("index({})", $schema);
        let index = define_archive!(@check,
            Self::read_resource(&*$storage, stringify!($index), &index_schema),
            $optional
        );
        let data = define_archive!(@check,
            Self::read_resource(&*$storage, stringify!($name), $schema),
            $optional
        );
        define_archive!(@if,
            $optional,
            {
                match (index, data) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                }
            },
            { (index, data) }
        )
    }};
    (@read, $storage:ident, archive($name:ident, $optional:ident, $schema:expr, $type:path, $builder_type:path)) => {{
        type Archive = $type;
        define_archive!(@check,
            Archive::open($storage.subdir(&stringify!($name))),
            $optional
        )
    }};

    // resource getters
    (@get, raw_data($name:ident, true, $schema:expr, $setter:ident)) => {
        #[inline]
        pub fn $name(&self) -> Option<$crate::RawData> {
            self.$name.as_ref().map(|mem_desc| $crate::RawData::new({unsafe{mem_desc.as_bytes()}}))
        }
    };
    (@get, raw_data($name:ident, false, $schema:expr, $setter:ident)) => {
        #[inline]
        pub fn $name(&self) -> $crate::RawData {
            $crate::RawData::new(unsafe {self.$name.as_bytes()})
        }
    };
    (@get, struct($name:ident, true, $schema:expr, $setter:ident, $type:path)) => {
        #[inline]
        pub fn $name(&self) -> Option<<$type as $crate::Struct>::Item>
        {
            self.$name.as_ref().map(|mem_desc| {<$type as $crate::Struct>::create(&unsafe{mem_desc.as_bytes()})})
        }
    };
    (@get, struct($name:ident, false, $schema:expr, $setter:ident, $type:path)) => {
        #[inline]
        pub fn $name(&self) -> <$type as $crate::Struct>::Item
        {
            <$type as $crate::Struct>::create(&unsafe{self.$name.as_bytes()})
        }
    };
    (@get, vector($name:ident, true, $schema:expr, $setter:ident, $starter:ident, $type:path)) => {
        #[inline]
        pub fn $name(&self) -> Option<$crate::ArrayView<$type>>
        {
            self.$name.as_ref().map(|x|$crate::ArrayView::new(unsafe{x.as_bytes()}))
        }
    };
    (@get, vector($name:ident, false, $schema:expr, $setter:ident, $starter:ident, $type:path)) => {
        #[inline]
        pub fn $name(&self) -> $crate::ArrayView<$type>
        {
            $crate::ArrayView::new(&unsafe{self.$name.as_bytes()})
        }
    };
    (@get, multivector($name:ident, true, $schema:expr, $starter:ident, $variadic_type:path, $index:ident, $index_type:path)) => {
        #[inline]
        pub fn $name(&self) -> Option<$crate::MultiArrayView<$variadic_type>>
        {
            self.$name.as_ref()
                .map(|(index, data)|{
                    $crate::MultiArrayView::new($crate::ArrayView::new(unsafe{index.as_bytes()}), unsafe{data.as_bytes()})
                })
        }
    };
    (@get, multivector($name:ident, false, $schema:expr, $starter:ident, $variadic_type:path, $index:ident, $index_type:path)) => {
        #[inline]
        pub fn $name(&self) -> $crate::MultiArrayView<$variadic_type>
        {
            $crate::MultiArrayView::new(
                $crate::ArrayView::new(&unsafe{self.$name.0.as_bytes()}),
                &unsafe{self.$name.1.as_bytes()},
            )
        }
    };
    (@get, archive($name:ident, true, $schema:expr, $type:path, $builder_type:path)) => {
        #[inline]
        pub fn $name(&self) -> Option<&$type>
        {
            self.$name.as_ref()
        }
    };
    (@get, archive($name:ident, false, $schema:expr, $type:path, $builder_type:path)) => {
        #[inline]
        pub fn $name(&self) -> &$type
        {
            &self.$name
        }
    };

    // resource setters
    (@set, raw_data($name:ident, $optional:ident, $schema:expr, $setter:ident)) => {
        #[inline]
        pub fn $setter(&self, data: &[u8]) -> ::std::io::Result<()> {
            self.storage.write(stringify!($name), $schema, data)
        }
    };
    (@set, struct($name:ident, $optional:ident, $schema:expr, $setter:ident, $type:path)) => {
        #[inline]
        pub fn $setter(&self, resource: <$type as $crate::Struct>::Item) -> ::std::io::Result<()> {
            let data = unsafe {
                ::std::slice::from_raw_parts(resource.as_ptr(), <$type as $crate::Struct>::SIZE_IN_BYTES)
            };
            self.storage.write(stringify!($name), $schema, data)
        }
    };
    (@set, vector($name:ident, $optional:ident, $schema:expr, $setter:ident, $starter:ident, $type:path)) => {
        #[inline]
        pub fn $setter(&self, vector: &$crate::ArrayView<$type>) -> ::std::io::Result<()> {
            self.storage.write(stringify!($name), $schema, vector.as_ref())
        }

        #[inline]
        pub fn $starter(&self) -> ::std::io::Result<$crate::ExternalVector<$type>> {
            $crate::create_external_vector(&*self.storage, stringify!($name), $schema)
        }
    };
    (@set, multivector($name:ident, $optional:ident, $schema:expr, $starter:ident, $variadic_type:path, $index:ident, $index_type:path)) => {
        #[inline]
        pub fn $starter(&self) -> ::std::io::Result<$crate::MultiVector<$variadic_type>> {
            $crate::create_multi_vector(&*self.storage, stringify!($name), $schema)
        }
    };
    (@set, archive($name:ident, $optional:ident, $schema:expr, $type:path, $builder_type:path)) => {
        #[inline]
        pub fn $name(&self) -> Result<$builder_type, $crate::ResourceStorageError> {
            use $crate::ArchiveBuilder;
            let storage = self.storage.subdir(stringify!($name));
            type Builder = $builder_type;
            Builder::new(storage)
        }
    };

    // main entry point
    ($name:ident, $builder_name:ident, $archive_schema:expr;
        $($resource_type:ident($resource_name:ident, $is_optional:ident $($args:tt)* ),)*
    ) => {
        #[derive(Clone)]
        pub struct $name {
            _storage: ::std::rc::Rc<dyn $crate::ResourceStorage>
            $(, $resource_name: define_archive!(@members, $resource_type($is_optional $($args)*)))*
        }

        impl $name {
            fn read_resource(
                storage: &$crate::ResourceStorage,
                name: &str,
                schema: &str,
            ) -> Result<$crate::MemoryDescriptor, $crate::ResourceStorageError>
            {
                storage.read(name, schema).map(|x| $crate::MemoryDescriptor::new(&x))
            }

            $(define_archive!(@get, $resource_type($resource_name, $is_optional $($args)* ));)*

            fn signature_name(archive_name: &str) -> String {
                format!("{}.archive", archive_name)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!(stringify!($name), " {{ ",
                        flatdata_intersperse!($(concat!(stringify!($resource_name), ": {:?}")),*),
                    " }}"),
                    $(self.$resource_name(), )*
                )
            }
        }

        impl $crate::Archive for $name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn open(storage: ::std::rc::Rc<dyn $crate::ResourceStorage>)
                -> ::std::result::Result<Self, $crate::ResourceStorageError>
            {
                storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

                $(let $resource_name = define_archive!(@read, storage, $resource_type($resource_name, $is_optional $($args)* ));)*

                Ok(Self {
                    _storage: storage
                    $(,$resource_name)*
                })
            }
        }

        #[derive(Clone, Debug)]
        pub struct $builder_name {
            storage: ::std::rc::Rc<dyn $crate::ResourceStorage>
        }

        impl $builder_name {
            $(define_archive!(@set, $resource_type($resource_name, $is_optional $($args)* ));)*
        }

        impl $crate::ArchiveBuilder for $builder_name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn new(
                storage: ::std::rc::Rc<dyn $crate::ResourceStorage>,
            ) -> Result<Self, $crate::ResourceStorageError> {
                $crate::create_archive::<Self>(&storage)?;
                Ok(Self { storage })
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[allow(warnings)]
    fn test_archive_compilation() {
        // This test checks that the archive definition below compiles.
        use crate::structs::Ref;

        define_struct!(
            A,
            RefA,
            RefMutA,
            "no_schema",
            4,
            (x, set_x, u32, u32, 0, 16),
            (y, set_y, u32, u32, 16, 16)
        );

        mod submodA {
            define_struct!(
                B,
                RefB,
                RefMutB,
                "no_schema",
                4,
                (x, set_x, u32, u32, 0, 16),
                (y, set_y, u32, u32, 16, 16)
            );
        }
        define_index!(
            IndexType32,
            RefIndexType32,
            RefMutIndexType32,
            "IndexType32 schema",
            4,
            32
        );

        define_variadic_struct!(Ts, RefTs, BuilderTs, IndexType32,
            0 => (A, A, add_a),
            1 => (B, submodA::B, add_b));

        define_archive!(SubArch, SubArchBuilder, "SubArch schema";
            raw_data(raw, false, "raw schema", set_raw),
        );

        mod submodB {
            pub const V_SCHEMA: &str = "v schema";
        }

        define_archive!(Arch, ArchBuilder, "Arch schema";
            struct(a, false, "a schema", set_a, A),
            struct(b, true, "b schema", set_b, submodA::B),
            vector(v, false, submodB::V_SCHEMA, set_v, start_v, A),
            vector(w, true, "w schema", set_w, start_w, A),
            multivector(mv, false, "mv schema", start_mv, Ts, mv_index, IndexType32),
            multivector(mw, true, "mw schema", start_mw, Ts, mw_index, IndexType32),
            raw_data(r, false, "r schema", set_r),
            raw_data(s, true, "s schema", set_s),
            archive(arch, false, "arch schema", SubArch, SubArchBuilder),
            archive(opt_arch, true, "opt_arch schema", SubArch, SubArchBuilder),
        );
    }
}

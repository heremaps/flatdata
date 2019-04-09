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
    fn open(storage: Rc<ResourceStorage>) -> Result<Self, ResourceStorageError>;
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
    fn new(storage: Rc<ResourceStorage>) -> Result<Self, ResourceStorageError>;
}

//
// Generator macros
//

/// Depending on the provided flag returns the type or wrap it in `Option`.
///
/// The flag is `true`, if the resource is optional.
#[doc(hidden)]
#[macro_export]
macro_rules! opt {
    ($type:ty, false) => {
        $type
    };
    ($type:ty, true) => {
        Option<$type>
    };
}

/// Depending on whether the first argument is `true` or `false` returns the
/// first block or the second, resp.
#[doc(hidden)]
#[macro_export]
macro_rules! static_if {
    (true, $true_block:block, $false_block:block) => {
        $true_block
    };
    (false, $true_block:block, $false_block:block) => {
        $false_block
    };
}

/// Depending on the provided flag returns the result or make it an `Option`.
///
/// The flag is `true`, if the resource is optional.
#[doc(hidden)]
#[macro_export]
macro_rules! check_resource {
    ($res:expr,false) => {
        $res?
    };
    ($res:expr,true) => {
        $res.ok()
    };
}

/// Macro used by generator to define a flatdata archive and corresponding
/// archive builder.
#[doc(hidden)]
#[macro_export]
macro_rules! define_archive {
    ($name:ident, $builder_name:ident, $archive_schema:expr;
        // struct resources
        $(($struct_resource:ident, $struct_setter:ident, $struct_type:path, $struct_schema:expr,
            $is_optional_struct:ident)),*;
        // vector resources
        $(($vector_resource:ident, $vector_setter:ident, $vector_start:ident,
            $element_type:path, $element_schema:expr, $is_optional_vector:ident)),*;
        // multivector resources
        $(($multivector_resource:ident,
            $multivector_start:ident,
            $variadic_type:path, $variadic_type_schema:expr,
            $multivector_resource_index:ident, $index_type:path,
            $is_optional_multivector:ident)),*;
        // raw data resources
        $(($raw_data_resource:ident, $raw_data_resource_setter:ident,
            $raw_data_schema:expr, $is_optional_raw_data:ident)),*;
        // subarchive resources
        $(($subarchive_resource:ident,
            $subarchive_type:path, $subarchive_builder_type:path, $subarchive_schema:expr,
            $is_optional_subarchive:ident)),*
    ) => {

        #[derive(Clone)]
        pub struct $name {
            _storage: ::std::rc::Rc<$crate::ResourceStorage>
            $(,$struct_resource: opt!($crate::MemoryDescriptor, $is_optional_struct))*
            $(,$vector_resource: opt!($crate::MemoryDescriptor, $is_optional_vector))*
            $(,$multivector_resource: (
                opt!($crate::MemoryDescriptor, $is_optional_multivector),
                opt!($crate::MemoryDescriptor, $is_optional_multivector)))*
            $(,$raw_data_resource: opt!($crate::MemoryDescriptor, $is_optional_raw_data))*
            $(,$subarchive_resource: opt!($subarchive_type, $is_optional_subarchive))*
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

            $(pub fn $struct_resource(&self) -> opt!(
                <$struct_type as $crate::Struct>::Item, $is_optional_struct)
            {
                static_if!($is_optional_struct, {
                    self.$struct_resource.as_ref().map(|mem_desc| {
                        <$struct_type as $crate::Struct>::create(&unsafe{mem_desc.as_bytes()})
                    })
                }, {
                    <$struct_type as $crate::Struct>::create(&unsafe{self.$struct_resource.as_bytes()})
                })
            })*

            $(pub fn $vector_resource(&self) -> opt!(
                $crate::ArrayView<$element_type>, $is_optional_vector)
            {
                static_if!($is_optional_vector, {
                    self.$vector_resource.as_ref().map(|x|$crate::ArrayView::new(unsafe{x.as_bytes()}))
                }, {
                    $crate::ArrayView::new(&unsafe{self.$vector_resource.as_bytes()})
                })
            })*

            $(pub fn $multivector_resource(&self) -> opt!(
                $crate::MultiArrayView<$variadic_type>, $is_optional_multivector)
            {
                static_if!($is_optional_multivector, {
                    let index_mem_desc = &self.$multivector_resource.0.as_ref();
                    let res_mem_desc = &self.$multivector_resource.1.as_ref();
                    index_mem_desc
                        .map(|x|$crate::ArrayView::new(unsafe{x.as_bytes()}))
                        .and_then(|index| {
                            res_mem_desc.map(|mem_desc| {
                                $crate::MultiArrayView::new(index, unsafe{mem_desc.as_bytes()})
                            })
                        })
                }, {
                    $crate::MultiArrayView::new(
                        $crate::ArrayView::new(&unsafe{self.$multivector_resource.0.as_bytes()}),
                        &unsafe{self.$multivector_resource.1.as_bytes()},
                    )
                })
            })*

            $(pub fn $raw_data_resource(&self) -> opt!($crate::RawData, $is_optional_raw_data) {
                static_if!($is_optional_raw_data, {
                    self.$raw_data_resource.as_ref().map(|mem_desc| {
                        $crate::RawData::new(unsafe { mem_desc.as_bytes() })
                    })
                }, {
                    $crate::RawData::new(unsafe { self.$raw_data_resource.as_bytes() })
                })
            })*

            $(pub fn $subarchive_resource(&self) -> opt!(
                &$subarchive_type, $is_optional_subarchive)
            {
                static_if!($is_optional_subarchive, {
                    self.$subarchive_resource.as_ref()
                }, {
                    &self.$subarchive_resource
                })
            })*

            fn signature_name(archive_name: &str) -> String {
                format!("{}.archive", archive_name)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!(stringify!($name), " {{ ",
                        intersperse!(""
                            $(, concat!(stringify!($struct_resource), ": {:?}"))*
                            $(, concat!(stringify!($vector_resource), ": {:?}"))*
                            $(, concat!(stringify!($multivector_resource), ": {:?}"))*
                            $(, concat!(stringify!($raw_data_resource), ": {:?}"))*
                            $(, concat!(stringify!($subarchive_resource), ": {:?}"))*
                        ),
                    " }}"),
                    $(self.$struct_resource(), )*
                    $(self.$vector_resource(), )*
                    $(self.$multivector_resource(), )*
                    $(self.$raw_data_resource, )*
                    $(self.$subarchive_resource, )*
                )
            }
        }

        impl $crate::Archive for $name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn open(storage: ::std::rc::Rc<$crate::ResourceStorage>)
                -> ::std::result::Result<Self, $crate::ResourceStorageError>
            {
                storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

                let read_resource = |name, schema| {
                    Self::read_resource(&*storage, name, schema )
                };

                $(let $struct_resource = check_resource!(
                    read_resource(stringify!($struct_resource), $struct_schema),
                    $is_optional_struct);
                )*

                $(let $vector_resource = check_resource!(
                    read_resource(stringify!($vector_resource), $element_schema),
                    $is_optional_vector);
                )*

                $(let index_schema = &format!("index({})", $variadic_type_schema);
                let $multivector_resource_index = check_resource!(
                    read_resource(stringify!($multivector_resource_index), &index_schema),
                    $is_optional_multivector);
                let $multivector_resource = check_resource!(
                    read_resource(stringify!($multivector_resource), $variadic_type_schema),
                    $is_optional_multivector);
                )*

                $(let $raw_data_resource = check_resource!(
                    read_resource(stringify!($raw_data_resource), $raw_data_schema),
                    $is_optional_raw_data);
                )*

                $(
                let $subarchive_resource = check_resource!(
                    {
                        type Archive = $subarchive_type;
                        Archive::open(storage.subdir(&stringify!($subarchive_resource)))
                    },
                    $is_optional_subarchive
                );)*

                Ok(Self {
                    _storage: storage
                    $(,$struct_resource)*
                    $(,$vector_resource)*
                    $(,$multivector_resource: (
                        $multivector_resource_index,
                        $multivector_resource))*
                    $(,$raw_data_resource)*
                    $(,$subarchive_resource)*
                })
            }
        }

        #[derive(Clone, Debug)]
        pub struct $builder_name {
            storage: ::std::rc::Rc<$crate::ResourceStorage>
        }

        impl $builder_name {
            $(pub fn $struct_setter(
                &self,
                resource: <$struct_type as $crate::Struct>::Item,
            ) -> ::std::io::Result<()> {
                let data = unsafe {
                    ::std::slice::from_raw_parts(resource.as_ptr(), <$struct_type as $crate::Struct>::SIZE_IN_BYTES)
                };
                self.storage
                    .write(stringify!($struct_resource), $struct_schema, data)
            })*

            $(pub fn $vector_setter(
                &self,
                vector: &$crate::ArrayView<$element_type>,
            ) -> ::std::io::Result<()> {
                self.storage
                    .write(stringify!($vector_resource), $element_schema, vector.as_ref())
            }

            pub fn $vector_start(
                &self,
            ) -> ::std::io::Result<$crate::ExternalVector<$element_type>> {
                $crate::create_external_vector(
                    &*self.storage,
                    stringify!($vector_resource),
                    $element_schema,
                )
            })*

            $(pub fn $multivector_start(
                &self,
            ) -> ::std::io::Result<
                $crate::MultiVector<$variadic_type>
            > {
                $crate::create_multi_vector(
                    &*self.storage,
                    stringify!($multivector_resource),
                    $variadic_type_schema,
                )
            })*

            $(pub fn $raw_data_resource_setter(&self, data: &[u8]) -> ::std::io::Result<()> {
                self.storage.write(
                    stringify!($raw_data_resource),
                    $raw_data_schema,
                    data,
                )
            })*

            $(pub fn $subarchive_resource(
                &self,
            ) -> Result<$subarchive_builder_type, $crate::ResourceStorageError> {
                use $crate::ArchiveBuilder;
                let storage = self.storage.subdir(stringify!($subarchive_resource));
                type Builder = $subarchive_builder_type;
                Builder::new(storage)
            }
            )*
        }

        impl $crate::ArchiveBuilder for $builder_name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn new(
                storage: ::std::rc::Rc<$crate::ResourceStorage>,
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

        define_struct!(
            B,
            RefB,
            RefMutB,
            "no_schema",
            4,
            (x, set_x, u32, u32, 0, 16),
            (y, set_y, u32, u32, 16, 16)
        );

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
            1 => (B, B, add_b));

        define_archive!(SubArch, SubArchBuilder, "SubArch schema";
            ;  // struct resources
            ;  // vector resources
            ;  // multivector resources
            (raw, set_raw, "raw schema", false) // raw data resources
            ; // subarchive resources
        );

        define_archive!(Arch, ArchBuilder, "Arch schema";
            // struct resources
            (a, set_a, A, "a schema", false),
            (b, set_b, A, "b schema", true);
            // vector resources
            (v, set_v, start_v, A, "v schema", false),
            (w, set_w, start_w, A, "w schema", true);
            // multivector resources
            (mv, start_mv, Ts, "mv schema", mv_index, IndexType32, false),
            (mw, start_mw, Ts, "mw schema", mw_index, IndexType32, true);
            // raw data resources
            (r, set_r, "r schema", false),
            (s, set_s, "s schema", true);
            // subarchive resources
            (arch, SubArch, SubArchBuilder, "arch schema", false),
            (opt_arch, SubArch, SubArchBuilder, "opt_arch schema", true)
        );
    }
}

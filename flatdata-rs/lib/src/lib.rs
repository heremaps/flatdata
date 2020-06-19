//! Implementation of [heremaps/flatdata] in Rust.
//!
//! Flatdata is a library providing data structures for convenient creation,
//! storage and access of packed memory-mappable structures with minimal
//! overhead.
//!
//! The idea is, that the user defines a schema of the data format using
//! flatdata's very simple schema language supporting plain structs, vectors
//! and multivectors. The schema is then used to generate builders and readers
//! for serialization and deserialization of the data. The data is serialized
//! in a portable way which allows zero-overhead random access to it by using
//! memory mapped storage. Memory mapped approach makes it possible to use the
//! operating system facilities for loading, caching and paging of the data,
//! and most important, accessing it as if it were in memory. Read more in
//! "[Why flatdata?]".
//!
//! This crate provides:
//!
//! * data structures for writing data to archives: [`StructBuf`], [`Vector`], [`ExternalVector`], [`MultiVector`]
//! * data structures for reading data from archives: [`ArrayView`], [`MultiArrayView`]
//! * resource storage backends for using archives: [`MemoryResourceStorage`], [`FileResourceStorage`]
//!
//! The generator is part of the main [heremaps/flatdata] repository,
//! the [`generate`] helper function is provided as a convenience wrapper.
//!
//! For a comprehensive example, cf. coappearances [schema] and the
//! corresponding [usage].
//!
//! # Examples
//!
//! First you design a schema for the data you want to store. Let's say we
//! want to store a list of prime factors for each natural number:
//!
//! ```flatdata
//! namespace prime {
//! // Represents a single prime factor of a number and how often it occurs.
//! struct Factor {
//!     value : u32 : 32;
//!     count : u32 : 8;
//! }
//!
//! // Points towards the beginning of the list of prime numbers.
//! struct Number {
//!     @range(factors)
//!     first_factor_ref : u32;
//! }
//!
//! // Stores a list of prime factors for numbers from 0 to N
//! archive Archive {
//!     @explicit_reference( Number.first_factor_ref, factors )
//!     numbers : vector<Number>;
//!
//!     factors : vector<Factor>;
//! }
//! }
//! ```
//!
//! Maybe create a diagram using the dot generator from [heremaps/flatdata]:
//!
//! ![diag][diag]
//!
//! Then you generate code using e.g. the [`generate`] utility in a `build.rs` script and include it in your project.
//! Now you can create a (disk-based) archive and fill it with data:
//!
//! ```rust,ignore
//! include!("prime_generated.rs");
//!
//! use flatdata::{ MemoryResourceStorage};
//!
//! pub fn calculate_prime_factors(
//!     builder: &mut prime::ArchiveBuilder,
//!     max_number: u32,
//! ) -> std::io::Result<()> {
//!     let mut numbers = builder.start_numbers()?;
//!     let mut factors = builder.start_factors()?;
//!     numbers.grow()?.set_first_factor_ref(0);
//!     for mut x in 0..=max_number {
//!         // Let's calculate prime factor in a very inefficient way
//!         for y in 2..x {
//!             let mut count = 0;
//!             while x % y == 0 {
//!                 count += 1;
//!                 x /= y;
//!             }
//!             if count > 0 {
//!                 let mut factor = factors.grow()?;
//!                 factor.set_value(y);
//!                 factor.set_count(count);
//!             }
//!         }
//!         numbers.grow()?.set_first_factor_ref(factors.len() as u32);
//!     }
//!     numbers.close().expect("Failed to close");
//!     factors.close().expect("Failed to close");
//!     Ok(())
//! }
//!
//! pub fn main() {
//!     let storage = MemoryResourceStorage::new("/primes");
//! let mut builder =
//!     prime::ArchiveBuilder::new(storage.clone()).expect("failed to create builder");
//! calculate_prime_factors(&mut builder, 10000).expect("Failed to write archive");
//! // store archive for re-use
//! // ...
//! // in a different application open archive for use:
//! let archive = prime::Archive::open(storage).expect("failed to open archive");
//! let number = 1234;
//! let factor_range = archive.numbers().at(number).first_factor_ref() as usize
//!     ..archive.numbers().at(number + 1).first_factor_ref() as usize;
//! let factors: Vec<_> = archive
//!     .factors()
//!     .slice(factor_range)
//!     .iter()
//!     .flat_map(|x| std::iter::repeat(x.value()).take(x.count() as usize))
//!     .collect();
//! println!("List if prime factors for {}: {:?}", number, factors);
//! }
//! ```
//!
//! This will print
//!
//! ```text
//! List if prime factors for 1234 is [2, 617]
//! ```
//!
//! [heremaps/flatdata]: https://github.com/heremaps/flatdata
//! [schema]: https://github.com/heremaps/flatdata/blob/master/examples/coappearances/coappearances.flatdata
//! [usage]: https://github.com/heremaps/flatdata/blob/master/flatdata-rs/tests/coappearances/src/lib.rs
//! [Why flatdata?]: https://github.com/heremaps/flatdata/blob/master/docs/why-flatdata.md
//! [`MemoryResourceStorage`]: struct.MemoryResourceStorage.html
//! [`FileResourceStorage`]: struct.FileResourceStorage.html
//! [`StructBuf`]: struct.StructBuf.html
//! [`Vector`]: struct.Vector.html
//! [`ExternalVector`]: struct.ExternalVector.html
//! [`MultiVector`]: struct.MultiVector.html
//! [`ArrayView`]: struct.ArrayView.html
//! [`MultiArrayView`]: struct.MultiArrayView.html
//! [`generate`]: fn.generate.html
//! [diag]: data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8%2BCjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIKICJodHRwOi8vd3d3LnczLm9yZy9HcmFwaGljcy9TVkcvMS4xL0RURC9zdmcxMS5kdGQiPgo8IS0tIEdlbmVyYXRlZCBieSBncmFwaHZpeiB2ZXJzaW9uIDIuNDAuMSAoMjAxNjEyMjUuMDMwNCkKIC0tPgo8IS0tIFRpdGxlOiBGbGF0ZGF0YURvdCBQYWdlczogMSAtLT4KPHN2ZyB3aWR0aD0iNTUycHQiIGhlaWdodD0iMTk4cHQiCiB2aWV3Qm94PSIwLjAwIDAuMDAgNTUyLjAwIDE5OC4wMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayI%2BCjxnIGlkPSJncmFwaDAiIGNsYXNzPSJncmFwaCIgdHJhbnNmb3JtPSJzY2FsZSgxIDEpIHJvdGF0ZSgwKSB0cmFuc2xhdGUoNCAxOTQpIj4KPHRpdGxlPkZsYXRkYXRhRG90PC90aXRsZT4KPHBvbHlnb24gZmlsbD0iI2ZmZmZmZiIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgcG9pbnRzPSItNCw0IC00LC0xOTQgNTQ4LC0xOTQgNTQ4LDQgLTQsNCIvPgo8ZyBpZD0iY2x1c3QxIiBjbGFzcz0iY2x1c3RlciI%2BCjx0aXRsZT5jbHVzdGVyX19wcmltZTwvdGl0bGU%2BCjxwb2x5Z29uIGZpbGw9IiNmN2Y3ZjciIHN0cm9rZT0iIzAwMDAwMCIgc3Ryb2tlLXdpZHRoPSIwIiBwb2ludHM9IjgsLTggOCwtMTgyIDUzNiwtMTgyIDUzNiwtOCA4LC04Ii8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMjQ3LjUiIHk9Ii0xNjYuMiIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXdlaWdodD0iYm9sZCIgZm9udC1zaXplPSIxNi4wMCIgZmlsbD0iIzUxNmQ3YiI%2BcHJpbWU8L3RleHQ%2BCjwvZz4KPGcgaWQ9ImNsdXN0MiIgY2xhc3M9ImNsdXN0ZXIiPgo8dGl0bGU%2BY2x1c3Rlcl9fcHJpbWVfQXJjaGl2ZTwvdGl0bGU%2BCjxwb2x5Z29uIGZpbGw9IiNlYmY4ZmYiIHN0cm9rZT0iIzg1ZDRmZiIgcG9pbnRzPSIxNiwtMTYgMTYsLTE0OCA1MjgsLTE0OCA1MjgsLTE2IDE2LC0xNiIvPgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjIzNy41IiB5PSItMTMyLjIiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC13ZWlnaHQ9ImJvbGQiIGZvbnQtc2l6ZT0iMTYuMDAiIGZpbGw9IiM1MTZkN2IiPkFyY2hpdmU8L3RleHQ%2BCjwvZz4KPGcgaWQ9ImNsdXN0MyIgY2xhc3M9ImNsdXN0ZXIiPgo8dGl0bGU%2BY2x1c3Rlcl9fcHJpbWVfQXJjaGl2ZV9udW1iZXJzPC90aXRsZT4KPHBvbHlnb24gZmlsbD0iI2M0ZTZmOCIgc3Ryb2tlPSIjODVkNGZmIiBzdHJva2Utd2lkdGg9IjAiIHBvaW50cz0iMjQsLTM2IDI0LC0xMTQgMjkxLC0xMTQgMjkxLC0zNiAyNCwtMzYiLz4KPHRleHQgdGV4dC1hbmNob3I9InN0YXJ0IiB4PSIxMzkiIHk9Ii0xMDMuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXdlaWdodD0iYm9sZCIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjNTE2ZDdiIj5udW1iZXJzPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjE0MS41IiB5PSItOTQuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXN0eWxlPSJpdGFsaWMiIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzUxNmQ3YiI%2BVmVjdG9yPC90ZXh0Pgo8L2c%2BCjxnIGlkPSJjbHVzdDQiIGNsYXNzPSJjbHVzdGVyIj4KPHRpdGxlPmNsdXN0ZXJfX3ByaW1lX0FyY2hpdmVfZmFjdG9yczwvdGl0bGU%2BCjxwb2x5Z29uIGZpbGw9IiNjNGU2ZjgiIHN0cm9rZT0iIzg1ZDRmZiIgc3Ryb2tlLXdpZHRoPSIwIiBwb2ludHM9IjMxMSwtMjQgMzExLC0xMTQgNTIwLC0xMTQgNTIwLC0yNCAzMTEsLTI0Ii8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMzk3IiB5PSItMTAzLjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC13ZWlnaHQ9ImJvbGQiIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzUxNmQ3YiI%2BZmFjdG9yczwvdGV4dD4KPHRleHQgdGV4dC1hbmNob3I9InN0YXJ0IiB4PSIzOTkuNSIgeT0iLTk0LjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zdHlsZT0iaXRhbGljIiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiM1MTZkN2IiPlZlY3RvcjwvdGV4dD4KPC9nPgo8IS0tIF9wcmltZV9BcmNoaXZlX251bWJlcnNfcHJpbWVfTnVtYmVyIC0tPgo8ZyBpZD0ibm9kZTEiIGNsYXNzPSJub2RlIj4KPHRpdGxlPl9wcmltZV9BcmNoaXZlX251bWJlcnNfcHJpbWVfTnVtYmVyPC90aXRsZT4KPHBvbHlnb24gZmlsbD0iIzI1N2ZhZCIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgcG9pbnRzPSI0MS41LC02MiA0MS41LC03NCAyNzQuNSwtNzQgMjc0LjUsLTYyIDQxLjUsLTYyIi8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iODguNSIgeT0iLTY2LjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjMDAwMDAwIj4gJiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7PC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjE1Mi41IiB5PSItNjYuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXdlaWdodD0iYm9sZCIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjZWJmOGZmIj5OdW1iZXI8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMTg0LjUiIHk9Ii02Ni44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzAwMDAwMCI%2BICYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOzwvdGV4dD4KPHBvbHlnb24gZmlsbD0iI2ViZjhmZiIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgcG9pbnRzPSI0MS41LC00OSA0MS41LC02MSAyNzQuNSwtNjEgMjc0LjUsLTQ5IDQxLjUsLTQ5Ii8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iNDIuNSIgeT0iLTUzLjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjMDAwMDAwIj4gJiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7PC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjEwNi41IiB5PSItNTMuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXdlaWdodD0iYm9sZCIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjNTE2ZDdiIj5maXJzdF9mYWN0b3JfcmVmPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjE5MS41IiB5PSItNTMuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPjo8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMTk3LjUiIHk9Ii01My44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzU2OGMzYiI%2BdTMyPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjIxMy41IiB5PSItNTMuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPjo8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMjE5LjUiIHk9Ii01My44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iI2QyMmQ3MiI%2BMzI8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMjMwLjUiIHk9Ii01My44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzAwMDAwMCI%2BICYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOzwvdGV4dD4KPC9nPgo8IS0tIF9wcmltZV9BcmNoaXZlX2ZhY3RvcnNfcHJpbWVfRmFjdG9yIC0tPgo8ZyBpZD0ibm9kZTIiIGNsYXNzPSJub2RlIj4KPHRpdGxlPl9wcmltZV9BcmNoaXZlX2ZhY3RvcnNfcHJpbWVfRmFjdG9yPC90aXRsZT4KPHBvbHlnb24gZmlsbD0iIzI1N2ZhZCIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgcG9pbnRzPSIzMjguNSwtNjMgMzI4LjUsLTc1IDUwMy41LC03NSA1MDMuNSwtNjMgMzI4LjUsLTYzIi8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMzQ2LjUiIHk9Ii02Ny44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzAwMDAwMCI%2BICYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOzwvdGV4dD4KPHRleHQgdGV4dC1hbmNob3I9InN0YXJ0IiB4PSI0MTAuNSIgeT0iLTY3LjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC13ZWlnaHQ9ImJvbGQiIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iI2ViZjhmZiI%2BRmFjdG9yPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQ0Mi41IiB5PSItNjcuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPiAmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDs8L3RleHQ%2BCjxwb2x5Z29uIGZpbGw9IiNlYmY4ZmYiIHN0cm9rZT0idHJhbnNwYXJlbnQiIHBvaW50cz0iMzI4LjUsLTUwIDMyOC41LC02MiA1MDMuNSwtNjIgNTAzLjUsLTUwIDMyOC41LC01MCIvPgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjMyOS41IiB5PSItNTQuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPiAmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDs8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMzkzLjUiIHk9Ii01NC44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtd2VpZ2h0PSJib2xkIiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiM1MTZkN2IiPnZhbHVlPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQyMC41IiB5PSItNTQuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPjo8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iNDI2LjUiIHk9Ii01NC44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzU2OGMzYiI%2BdTMyPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQ0Mi41IiB5PSItNTQuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPjo8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iNDQ4LjUiIHk9Ii01NC44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iI2QyMmQ3MiI%2BMzI8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iNDU5LjUiIHk9Ii01NC44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzAwMDAwMCI%2BICYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOyYjMTYwOzwvdGV4dD4KPHBvbHlnb24gZmlsbD0iI2ViZjhmZiIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgcG9pbnRzPSIzMjguNSwtMzcgMzI4LjUsLTQ5IDUwMy41LC00OSA1MDMuNSwtMzcgMzI4LjUsLTM3Ii8%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMzMyIiB5PSItNDEuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXNpemU9IjkuMDAiIGZpbGw9IiMwMDAwMDAiPiAmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDsmIzE2MDs8L3RleHQ%2BCjx0ZXh0IHRleHQtYW5jaG9yPSJzdGFydCIgeD0iMzk2IiB5PSItNDEuOCIgZm9udC1mYW1pbHk9IkNvdXJpZXIgTmV3IiBmb250LXdlaWdodD0iYm9sZCIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjNTE2ZDdiIj5jb3VudDwvdGV4dD4KPHRleHQgdGV4dC1hbmNob3I9InN0YXJ0IiB4PSI0MjMiIHk9Ii00MS44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzAwMDAwMCI%2BOjwvdGV4dD4KPHRleHQgdGV4dC1hbmNob3I9InN0YXJ0IiB4PSI0MjkiIHk9Ii00MS44IiBmb250LWZhbWlseT0iQ291cmllciBOZXciIGZvbnQtc2l6ZT0iOS4wMCIgZmlsbD0iIzU2OGMzYiI%2BdTMyPC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQ0NSIgeT0iLTQxLjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjMDAwMDAwIj46PC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQ1MSIgeT0iLTQxLjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjZDIyZDcyIj44PC90ZXh0Pgo8dGV4dCB0ZXh0LWFuY2hvcj0ic3RhcnQiIHg9IjQ1NyIgeT0iLTQxLjgiIGZvbnQtZmFtaWx5PSJDb3VyaWVyIE5ldyIgZm9udC1zaXplPSI5LjAwIiBmaWxsPSIjMDAwMDAwIj4gJiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7JiMxNjA7PC90ZXh0Pgo8L2c%2BCjwhLS0gX3ByaW1lX0FyY2hpdmVfbnVtYmVyc19wcmltZV9OdW1iZXImIzQ1OyZndDtfcHJpbWVfQXJjaGl2ZV9mYWN0b3JzX3ByaW1lX0ZhY3RvciAtLT4KPGcgaWQ9ImVkZ2UxIiBjbGFzcz0iZWRnZSI%2BCjx0aXRsZT5fcHJpbWVfQXJjaGl2ZV9udW1iZXJzX3ByaW1lX051bWJlcjpwb3J0X19wcmltZV9BcmNoaXZlX251bWJlcnNfcHJpbWVfTnVtYmVyX2ZpcnN0X2ZhY3Rvcl9yZWYmIzQ1OyZndDtfcHJpbWVfQXJjaGl2ZV9mYWN0b3JzX3ByaW1lX0ZhY3RvcjwvdGl0bGU%2BCjxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iIzI1N2ZhZCIgZD0iTTI3OC41NDgzLC01NS4wMDE0QzI5MC4wMTgsLTU1LjAwOSAzMDEuOTM1OCwtNTUuMDQ4IDMxMy43NDI2LC01NS4xMDY1Ii8%2BCjxlbGxpcHNlIGZpbGw9IiMyNTdmYWQiIHN0cm9rZT0iIzI1N2ZhZCIgY3g9IjI3Ni41IiBjeT0iLTU1LjAwMDciIHJ4PSIyIiByeT0iMiIvPgo8cG9seWdvbiBmaWxsPSIjMjU3ZmFkIiBzdHJva2U9IiMyNTdmYWQiIHBvaW50cz0iMzEzLjc4NTEsLTU2Ljg1NjcgMzE4Ljc5NDIsLTU1LjEzMjggMzEzLjgwMzQsLTUzLjM1NjcgMzEzLjc4NTEsLTU2Ljg1NjciLz4KPC9nPgo8L2c%2BCjwvc3ZnPg%3D%3D
#![deny(missing_docs, missing_debug_implementations, warnings)]

/// Number of elements in `ArrayView`, `MultiArrayView`, and `Vector` to show
/// in Debug output.
const DEBUG_PREVIEW_LEN: usize = 10;

#[macro_use]
mod bytereader;
#[macro_use]
mod bytewriter;

mod arrayview;
mod error;
mod filestorage;
mod generator;
mod memory;
mod memstorage;
mod multiarrayview;
mod multivector;
mod rawdata;
mod storage;
mod structs;
mod vector;

#[doc(hidden)]
pub mod helper;

#[doc(hidden)]
pub mod test;

pub use crate::{
    arrayview::SliceExt,
    error::*,
    filestorage::FileResourceStorage,
    generator::*,
    memory::PADDING_SIZE,
    memstorage::MemoryResourceStorage,
    multiarrayview::MultiArrayView,
    multivector::MultiVector,
    rawdata::RawData,
    storage::{
        create_archive, create_external_vector, create_multi_vector, ResourceStorage, StorageHandle,
    },
    structs::*,
    vector::*,
};

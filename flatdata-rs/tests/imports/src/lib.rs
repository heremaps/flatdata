#![cfg(test)]
#![allow(dead_code)]

// Modules from the "simple" import test case:
// main.flatdata imports types.flatdata, so main's generated code uses
// `pub use crate::types::n::*;`
pub mod types;
pub mod simple_main;

// Modules from the "cross_namespace" import test case:
// main.flatdata imports other.flatdata, so main's generated code uses
// `pub use crate::other::defs::*;`
pub mod other;
pub mod cross_ns_main;

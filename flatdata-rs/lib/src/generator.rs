use std::{
    io::Write,
    path::{Path, PathBuf},
};

/// A helper function wrapping the flatdata generator.
///
/// Can be used to write build.rs build scripts, generating outputs either from
/// a single schema, or recursively from a folder of schemas.
///
/// `schemas_path` can either be a single file, or a filder containing schemas.
/// In both cases the function will only handle files with the `.flatdata`
/// extension.
///
/// Generated files are in the same relative location, e.g.
/// ``` text
/// schemas_path/
/// ├────────────example_a/
/// │            ├─────────my_schema.flatdata
/// │            └─────────my_other_schema.flatdata
/// └────────────example_b.flatdata
/// ```
///
/// results in
/// ``` text
/// out_dir/
/// ├───────example_a/
/// │       ├─────────my_schema.rs
/// │       └─────────my_other_schema.rs
/// └───────example_b.rs
/// ```
///
/// # Examples
///
/// `build.rs`
/// ``` ignore
/// use std::path::PathBuf;
///
/// fn main() {
///     let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
///     let test_cases_path = PathBuf::from("assets/schemas");
///     let generator_path = PathBuf::from(std::env::var("GENERATOR_PATH").unwrap());
///
///     match flatdata::generate(&out_path, &test_cases_path, &generator_path) {
///        Err(e) => {
///            eprintln!("{}", e);
///            std::process::exit(1);
///        }
///        _ => (),
///     }
/// }
/// ```
///
/// `my_schema.rs`
/// ``` ignore
/// #![allow(dead_code)]
///
/// include!(concat!(env!("OUT_DIR"), "/path/to/my_schema.rs"));
///
/// // re-export if desired
/// pub use my_schema::*;
/// ```
pub fn generate(
    output_dir: &Path,
    schemas_path: &Path,
    generator: &Path,
) -> Result<(), GeneratorError> {
    let output_dir = output_dir.canonicalize()?;
    let schemas_path = schemas_path.canonicalize()?;
    let generator = generator.canonicalize()?;

    for entry in walkdir::WalkDir::new(&schemas_path) {
        let entry = entry?;
        if entry.path().extension().map_or(true, |x| x != "flatdata") {
            continue;
        }

        let result: PathBuf = if schemas_path.is_dir() {
            output_dir
                .join(entry.path().strip_prefix(&schemas_path)?)
                .with_extension("rs")
        } else {
            output_dir
                .join(entry.path().file_name().unwrap())
                .with_extension("rs")
        };

        std::fs::create_dir_all(result.parent().unwrap())?;
        let output = std::process::Command::new(&generator)
            .arg("-g")
            .arg("rust")
            .arg("-s")
            .arg(&entry.path())
            .arg("-O")
            .arg(&result)
            .output()?;

        std::io::stderr().write_all(&output.stderr).unwrap();

        if !output.status.success() {
            return Err(GeneratorError::Failure {
                schema: result,
                destination: entry.path().into(),
            });
        }

        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
    Ok(())
}

/// Error type for generate function
#[derive(Debug)]
pub enum GeneratorError {
    /// Wrapper around underlying io::Error
    Io(std::io::Error),
    /// Failed to compute paths
    Path(std::path::StripPrefixError),
    /// Failed to run generator
    Failure {
        /// path to the problematic schema
        schema: PathBuf,
        /// path to the generated file
        destination: PathBuf,
    },
}

impl std::fmt::Display for GeneratorError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            GeneratorError::Io(details) => write!(f, "GeneratorError::Io( details: {})", details),
            GeneratorError::Path(details) => {
                write!(f, "GeneratorError::Path( details: {})", details)
            }
            GeneratorError::Failure {
                schema,
                destination,
            } => write!(
                f,
                "GeneratorError::Failure {{\n    schema: {},\n    destination: {},\n}}",
                schema.display(),
                destination.display(),
            ),
        }
    }
}

impl std::error::Error for GeneratorError {}

impl std::convert::From<std::io::Error> for GeneratorError {
    fn from(detail: std::io::Error) -> Self {
        GeneratorError::Io(detail)
    }
}

impl std::convert::From<walkdir::Error> for GeneratorError {
    fn from(detail: walkdir::Error) -> Self {
        GeneratorError::Io(detail.into())
    }
}

impl std::convert::From<std::path::StripPrefixError> for GeneratorError {
    fn from(detail: std::path::StripPrefixError) -> Self {
        GeneratorError::Path(detail)
    }
}

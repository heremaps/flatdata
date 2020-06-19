use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
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
/// ## Examples
///
/// `build.rs`
///
/// ```ignore
/// use std::env;
///
/// fn main() {
///     flatdata::generate("schemas_path/", &env::var("OUT_DIR").unwrap()).unwrap();
/// }
/// ```
///
/// `my_schema.rs`
///
/// ```ignore
/// #![allow(dead_code)]
///
/// include!(concat!(env!("OUT_DIR"), "/example_a/my_schema.rs"));
///
/// // re-export if desired
/// pub use my_schema::*;
/// ```
///
/// ## Development
///
/// If you are working on the generator, you can make sure your `build.rs`
/// script picks up the source by setting `FLATDATA_GENERATOR_PATH` to point to
/// the `flatdata-generator` folder.
pub fn generate(
    schemas_path: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
) -> Result<(), GeneratorError> {
    let schemas_path = schemas_path.as_ref();
    let out_dir = out_dir.as_ref();

    // create a virtualenv in the target folder
    eprintln!("creating python virtualenv");
    let _ = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(out_dir)
        .spawn()
        .map_err(GeneratorError::PythonError)?
        .wait()?;

    // install dependencies
    let generator_path = if let Ok(path) = env::var("FLATDATA_GENERATOR_PATH") {
        // we want to rebuild automatically if we edit the generator's code
        let path = PathBuf::from(path).canonicalize()?;
        println!("cargo:rerun-if-changed={}", path.display());
        eprintln!("installing flatdata-generator from source");
        path
    } else {
        eprintln!("installing flatdata-generator from PyPI");
        PathBuf::from("flatdata-generator==0.4.1")
    };
    let _ = Command::new(out_dir.join("bin/pip3"))
        .arg("install")
        .arg("--disable-pip-version-check")
        .arg(&generator_path)
        .spawn()
        .map_err(GeneratorError::PythonError)?
        .wait()?;

    for entry in walkdir::WalkDir::new(&schemas_path) {
        let entry = entry?;
        if entry.path().extension().map_or(true, |x| x != "flatdata") {
            continue;
        }

        let result: PathBuf = if schemas_path.is_dir() {
            out_dir
                .join(entry.path().strip_prefix(&schemas_path).unwrap())
                .with_extension("rs")
        } else {
            out_dir
                .join(entry.path().file_name().unwrap())
                .with_extension("rs")
        };
        eprintln!(
            "generating {} from {}",
            result.display(),
            schemas_path.display()
        );

        std::fs::create_dir_all(result.parent().unwrap())?;
        let exit_code = Command::new(out_dir.join("bin/flatdata-generator"))
            .arg("-g")
            .arg("rust")
            .arg("-s")
            .arg(&entry.path())
            .arg("-O")
            .arg(&result)
            .spawn()
            .map_err(|e| GeneratorError::Failure {
                schema: entry.path().into(),
                destination: result.clone(),
                error: e,
            })?
            .wait()?
            .code();

        if exit_code != Some(0) {
            return Err(GeneratorError::Failure {
                schema: entry.path().into(),
                destination: result,
                error: io::Error::new(
                    io::ErrorKind::Other,
                    match exit_code {
                        Some(code) => format!("Failed to run the generator, exit code {}", code),
                        None => "Failed to run the generator, no exit code".into(),
                    },
                ),
            });
        }

        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
    Ok(())
}

/// Error type for generate function
#[derive(Debug)]
pub enum GeneratorError {
    /// Python interpreter or virtualenv not found
    PythonError(std::io::Error),
    /// Wrapper around underlying io::Error
    Io(std::io::Error),
    /// Failed to run generator
    Failure {
        /// path to the problematic schema
        schema: PathBuf,
        /// path to the generated file
        destination: PathBuf,
        /// the original io::Error
        error: std::io::Error,
    },
}

impl std::fmt::Display for GeneratorError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            GeneratorError::PythonError(err) => {
                writeln!(f, "{} could not be executed", err)?;
                writeln!(
                    f,
                    "Failed to prepare virtualenv for flatdata-generator: please make sure both python3 and python3-virtualenv are installed."
                )
            }
            GeneratorError::Io(details) => {
                write!(f, "Failed to run flatdata-generator: {}", details)
            }
            GeneratorError::Failure {
                schema,
                destination,
                error,
            } => write!(
                f,
                "Failed to run generate {} from {}: {}",
                schema.display(),
                destination.display(),
                error
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

use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

/// A helper function wrapping the flatdata generator.
///
/// Can be used to write build.rs build scripts, generating outputs either from a single schema,
/// or recursively from a folder of schemas.
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
///     flatdata::generate(&out_path, &test_cases_path, &generator_path);
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
pub fn generate(output_dir: &Path, schemas_path: &Path, generator: &Path) {
    let output_dir = output_dir.canonicalize().unwrap();
    let schemas_path = schemas_path.canonicalize().unwrap();
    let generator = generator.canonicalize().unwrap();

    for entry in walkdir::WalkDir::new(&schemas_path) {
        let entry = entry.expect("Cannot access file");
        if entry.path().extension().map_or(true, |x| x != "flatdata") {
            continue;
        }

        let result: PathBuf = if schemas_path.is_dir() {
            output_dir
                .join(entry.path().strip_prefix(&schemas_path).unwrap())
                .with_extension("rs")
        } else {
            output_dir
                .join(entry.path().file_name().unwrap())
                .with_extension("rs")
        };

        println!(
            "Generating {:?} from {:?}",
            result.display(),
            entry.path().display()
        );

        std::fs::create_dir_all(result.parent().unwrap()).expect("Failed to create output dir");
        let output = std::process::Command::new(&generator)
            .arg("-g")
            .arg("rust")
            .arg("-s")
            .arg(&entry.path())
            .arg("-O")
            .arg(&result)
            .output()
            .expect("Failed to run generator");

        std::io::stderr().write_all(&output.stderr).unwrap();
        assert!(
            output.status.success(),
            format!("Failed to generate: {:?}", output.status)
        );

        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
}

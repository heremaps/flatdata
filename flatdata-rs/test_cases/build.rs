use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let test_cases_path = dir.join("../../test_cases").canonicalize().unwrap();

    for entry in walkdir::WalkDir::new(&test_cases_path) {
        let entry = entry.expect("Cannot access file");
        if entry.path().extension().map_or(true, |x| x != "flatdata") {
            continue;
        }

        let mut result: PathBuf = entry.path().strip_prefix(&test_cases_path).unwrap().into();
        result.set_extension("rs");

        let filename = out_path.join(&result);
        std::fs::create_dir_all(filename.parent().unwrap()).expect("Failed to create output dir");
        let output = std::process::Command::new("python3")
            .arg("../../generator")
            .arg("-g")
            .arg("rust")
            .arg("-s")
            .arg(&entry.path())
            .arg("-O")
            .arg(&filename)
            .output()
            .expect("Failed to run generator");

        std::io::stderr().write_all(&output.stderr).unwrap();
        assert!(output.status.success());
    }
}

use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let schema_path = dir.join("../../../examples/coappearances/coappearances.flatdata");
    let filename = out_path.join("schema.rs");
    let output = std::process::Command::new("python3")
        .arg("../../../generator")
        .arg("-g")
        .arg("rust")
        .arg("-s")
        .arg(&schema_path)
        .arg("-O")
        .arg(&filename)
        .output()
        .expect("Failed to run generator");

    std::io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

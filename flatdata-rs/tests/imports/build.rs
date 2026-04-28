use std::path::Path;

fn main() {
    let test_cases = Path::new("../../../test_cases/imports");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Generate each schema file. The generator CLI uses Engine.from_file()
    // which resolves imports automatically.
    let schemas = [
        // simple: main imports types
        ("simple/types.flatdata", "simple/types.rs"),
        ("simple/main.flatdata", "simple/main.rs"),
        // cross_namespace: main imports other (different namespace)
        ("cross_namespace/other.flatdata", "cross_namespace/other.rs"),
        ("cross_namespace/main.flatdata", "cross_namespace/main.rs"),
    ];

    let generator_bin = find_generator(&out_dir);

    for (schema, output) in &schemas {
        let schema_path = test_cases.join(schema);
        let output_path = Path::new(&out_dir).join(output);
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();

        let status = std::process::Command::new(&generator_bin)
            .arg("-g")
            .arg("rust")
            .arg("-s")
            .arg(&schema_path)
            .arg("-O")
            .arg(&output_path)
            .status()
            .unwrap_or_else(|e| panic!("Failed to run generator for {}: {}", schema, e));

        assert!(
            status.success(),
            "Generator failed for {} with exit code {:?}",
            schema,
            status.code()
        );

        println!("cargo:rerun-if-changed={}", schema_path.display());
    }
}

fn find_generator(out_dir: &str) -> std::path::PathBuf {
    if let Ok(bin_path) = std::env::var("FLATDATA_GENERATOR_BIN") {
        return std::path::PathBuf::from(bin_path);
    }

    let out = Path::new(out_dir);

    let _ = std::process::Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(out)
        .status();

    let generator_path = if let Ok(path) = std::env::var("FLATDATA_GENERATOR_PATH") {
        std::path::PathBuf::from(path)
    } else {
        std::path::PathBuf::from("../../../flatdata-generator")
    };

    let _ = std::process::Command::new(out.join("bin/pip3"))
        .arg("install")
        .arg("--disable-pip-version-check")
        .arg(&generator_path)
        .status();

    out.join("bin/flatdata-generator")
}

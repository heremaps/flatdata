use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let test_cases_path = PathBuf::from("../../../test_cases");
    let generator_path = PathBuf::from("../../../generator");

    flatdata::generate(&out_path, &test_cases_path, &generator_path);
}

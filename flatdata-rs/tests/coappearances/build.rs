use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let test_cases_path = PathBuf::from("../../../examples/coappearances/coappearances.flatdata");
    let generator_path = PathBuf::from("../../../generator");

    match flatdata::generate(&out_path, &test_cases_path, &generator_path) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        _ => (),
    }
}

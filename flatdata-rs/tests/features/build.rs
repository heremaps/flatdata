fn main() {
    flatdata::generate("../../../test_cases", &std::env::var("OUT_DIR").unwrap())
        .expect("generator failed");
}

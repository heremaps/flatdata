fn main() {
    flatdata_gen::generate("../../../test_cases", &std::env::var("OUT_DIR").unwrap())
        .expect("generator failed");
}

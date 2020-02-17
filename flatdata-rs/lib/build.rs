fn main() {
    flatdata_gen::generate("src/test/test.flatdata", std::env::var("OUT_DIR").unwrap())
        .expect("generator failed");
}

use std::env;

fn main() {
    flatdata::generate("../../../test_cases", &env::var("OUT_DIR").unwrap()).unwrap();
}

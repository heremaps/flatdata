use std::env;

fn main() {
    flatdata::generate("../../../examples/coappearances/coappearances.flatdata", &env::var("OUT_DIR").unwrap()).unwrap();
}

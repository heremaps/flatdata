fn main() {
    flatdata::generate(
        "assets/coappearances.flatdata",
        &std::env::var("OUT_DIR").unwrap(),
    )
    .expect("generator failed");
}

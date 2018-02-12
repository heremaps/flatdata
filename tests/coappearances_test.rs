#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::path;
use std::rc::Rc;

use flatdata::Archive;

mod coappearances;

#[test]
fn read_coappearances() {
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        path::PathBuf::from("examples/coappearances/karenina.archive"),
    )));
    let g = coappearances::Graph::open(storage);
    assert!(g.is_ok());
    let g = g.unwrap();

    assert_eq!(g.vertices().size(), 138);
    assert_eq!(g.edges().size(), 494);
}

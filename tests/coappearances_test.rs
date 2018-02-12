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
        path::PathBuf::from("tests/coappearances/karenina.archive"),
    )));
    let g = coappearances::Graph::open(storage).expect("archive");

    assert_eq!(g.meta().title_ref(), 0);
    assert_eq!(g.meta().author_ref(), 14);
    assert_eq!(g.vertices().size(), 138);
    assert_eq!(g.edges().size(), 494);

    let num_chapters = g.edges().iter().map(|e| e.count() as usize).sum();
    assert_eq!(g.chapters().size(), num_chapters);
}

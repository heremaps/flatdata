#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::path;
use std::rc::Rc;
use std::str;

use flatdata::Archive;

mod coappearances;

fn substring(strings: &str, start: u32) -> &str {
    let start = start as usize;
    let end = strings[start..].find('\0').expect("invalid string");
    &strings[start..start + end]
}

#[test]
fn read_and_validate_coappearances() {
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        path::PathBuf::from("tests/coappearances/karenina.archive"),
    )));
    let g = coappearances::Graph::open(storage).expect("invalid archive");

    assert_eq!(g.vertices().size(), 138);
    assert_eq!(g.edges().size(), 494);

    // Note: We could use `from_utf8_unchecked` here, which simply does a raw pointer conversion,
    // however we use the safe version of the function, which does exactly the same except it
    // validates that the string is utf8. For that, it need to scan the whole string once, which is
    // usually undesirable for huge data.
    let strings = str::from_utf8(g.strings()).expect("invalid utf8 string");

    assert_eq!(substring(strings, g.meta().title_ref()), "Anna Karenina");
    assert_eq!(substring(strings, g.meta().author_ref()), "Leo Tolstoy");

    let num_chapters = g.edges().iter().map(|e| e.count() as usize).sum();
    assert_eq!(g.chapters().size(), num_chapters);
}

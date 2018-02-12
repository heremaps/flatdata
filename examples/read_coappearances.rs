#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::env;
use std::path;
use std::process;
use std::rc::Rc;

use flatdata::Archive;

mod coappearances;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: read_coappearances <flatdata_archive>");
        process::exit(1);
    });

    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        path::PathBuf::from(path),
    )));
    let g = coappearances::Graph::open(storage).unwrap_or_else(|e| {
        match e {
            flatdata::ResourceStorageError::WrongSignature {
                resource_name,
                diff,
            } => eprintln!(
                "[E] Could not open graph archive: WrongSignature of {}\nDiff:\n{}",
                resource_name, diff
            ),
            e => eprintln!("[E] Could not open graph archive: {}", e),
        };
        process::exit(1);
    });
    println!("{:?}", g);

    for v in g.vertices().iter() {
        println!("{:?}", v);
    }

    for e in g.edges().iter() {
        println!("{:?}", e);
    }
}

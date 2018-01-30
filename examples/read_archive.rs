#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::env;
use std::path;
use std::process;
use std::rc::Rc;

use flatdata::Archive;

mod graph;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} <flatdata_archive>", prog_name);
        process::exit(1);
    });

    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        path::PathBuf::from(path),
    )));
    let g = graph::Graph::open(storage).unwrap_or_else(|e| {
        eprintln!("Could not open graph archive due to {}", e);
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

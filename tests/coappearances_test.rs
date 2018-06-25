#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::Read;
use std::path;
use std::rc::Rc;
use std::str;

use flatdata::{Archive, ArchiveBuilder};

pub mod coappearances;

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
    println!("{:?}", g);

    let vertices = g.vertices();
    let edges = g.edges();
    assert_eq!(vertices.len(), 138);
    assert_eq!(edges.len(), 494);

    // Note: We could use `from_utf8_unchecked` here, which simply does a raw
    // pointer conversion, however we use the safe version of the function,
    // which does exactly the same except it validates that the string is utf8.
    // For that, it need to scan the whole string once, which is
    // usually undesirable for huge data.
    let strings = str::from_utf8(g.strings()).expect("invalid utf8 string");

    let meta = g.meta();
    assert_eq!(meta, g.meta());

    assert_eq!(
        substring(strings, meta.title_ref()),
        "Anna Karenina (Анна Каренина)"
    );
    assert_eq!(
        substring(strings, meta.author_ref()),
        "Leo Tolstoy (Лев Николаевич Толстой)"
    );

    let num_chapters = edges.iter().map(|e| e.count() as usize).sum();
    assert_eq!(g.chapters().len(), num_chapters);

    assert_eq!(substring(strings, vertices.at(0).name_ref()), "Annushka");
    assert_eq!(
        substring(strings, vertices.at(3).name_ref()),
        "Anna Arkadyevna Karenina"
    );

    let e0 = edges.at(0);
    assert_eq!(
        substring(strings, vertices.at(e0.a_ref() as usize).name_ref()),
        "Annushka"
    );
    assert_eq!(
        substring(strings, vertices.at(e0.b_ref() as usize).name_ref()),
        "Anna Arkadyevna Karenina"
    );

    let validate_chapters = |edge_ref, expected| {
        let e = edges.at(edge_ref);
        let chapters_start = e.first_chapter_ref() as usize;
        let chapters_end = edges.at(edge_ref + 1).first_chapter_ref() as usize;
        let e_chapters: Vec<String> = g
            .chapters()
            .iter()
            .skip(chapters_start)
            .take(chapters_end - chapters_start)
            .map(|ch| format!("{}.{}", ch.major(), ch.minor()))
            .collect();
        assert_eq!(e_chapters, expected);
    };

    validate_chapters(
        0,
        vec![
            "1.29", "2.27", "3.15", "3.16", "6.19", "7.25", "7.27", "7.29",
        ],
    );
    validate_chapters(1, vec!["6.19"]);
    // Note: Last element in edges is not an edge but a sentinel which allows us to
    // calculate the range of chapter.
    validate_chapters(edges.len() - 2, vec!["7.25"]);

    let vertices_data = g.vertices_data();
    assert_eq!(vertices_data.len(), vertices.len());

    let data: Vec<_> = vertices_data.at(0).collect();
    assert_eq!(data.len(), 1);
    match *data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "maid");
            assert_eq!(
                substring(strings, vertices.at(data.to_ref() as usize).name_ref()),
                "Anna Arkadyevna Karenina"
            );
        }
        _ => assert!(false),
    };

    let data: Vec<_> = vertices_data.at(1).collect();
    assert_eq!(data.len(), 1);
    match *data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "housekeeper");
            assert_eq!(
                substring(strings, vertices.at(data.to_ref() as usize).name_ref()),
                "Konstantin Dmitrievitch Levin"
            );
        }
        _ => assert!(false),
    };

    let data: Vec<_> = vertices_data.at(vertices_data.len() - 1).collect();
    assert_eq!(data.len(), 1);
    match *data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "gambling friend");
            assert_eq!(
                substring(strings, vertices.at(data.to_ref() as usize).name_ref()),
                "Count Alexey Kirillovitch Vronsky"
            );
        }
        _ => assert!(false),
    };
}

fn compare_files(name_a: &path::Path, name_b: &path::Path) -> bool {
    let mut fa = fs::File::open(name_a).unwrap();
    let mut buf_a = Vec::new();
    fa.read_to_end(&mut buf_a).unwrap();

    let mut fb = fs::File::open(name_b).unwrap();
    let mut buf_b = Vec::new();
    fb.read_to_end(&mut buf_b).unwrap();

    buf_a == buf_b
}

fn compare_resource(from: &path::PathBuf, to: &path::PathBuf, resource_name: &str) -> bool {
    compare_files(&from.join(resource_name), &to.join(resource_name))
        && compare_files(
            &from.join(format!("{}.schema", resource_name)),
            &to.join(format!("{}.schema", resource_name)),
        )
}

fn copy_coappearances_archive(
    from_path: &str,
    to_path: &str,
) -> (path::PathBuf, coappearances::GraphBuilder) {
    // open for reading
    let source_archive_path = path::PathBuf::from(from_path);
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        source_archive_path.clone(),
    )));
    let g = coappearances::Graph::open(storage).expect("invalid archive");

    // open for writing
    let archive_path = env::temp_dir().join(to_path);
    println!("Will create archive in: {}", archive_path.to_str().unwrap());
    if archive_path.exists() {
        fs::remove_dir_all(&archive_path).expect("could not remove already existing archive");
    }
    fs::create_dir_all(&archive_path).expect("could not create archive dir");
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        archive_path.clone(),
    )));
    let mut gb = coappearances::GraphBuilder::new(storage).expect("could not create archive");

    // copy data
    let mut meta = flatdata::StructBuf::<coappearances::Meta>::new();
    meta.fill_from(&g.meta());
    gb.set_meta(&meta).expect("set_meta failed");
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "meta"
    ));

    let mut vertices = gb.start_vertices().expect("start_vertices failed");
    for v in g.vertices().iter() {
        let mut w = vertices.grow().expect("grow failed");
        w.fill_from(&v);
    }
    vertices.close().expect("close failed");
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "vertices"
    ));

    let mut edges = flatdata::Vector::<coappearances::Coappearance>::new();
    for e in g.edges().iter() {
        edges.grow().fill_from(&e);
    }
    gb.set_edges(&edges.as_view()).expect("set_edges failed");
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "edges"
    ));

    let mut vertices_data = gb
        .start_vertices_data()
        .expect("start_vertices_data failed");
    for item in g.vertices_data().iter() {
        let mut new_item = vertices_data.grow().expect("grow failed");
        for element in item {
            match *element {
                coappearances::VerticesData::Nickname(ref nickname) => {
                    let mut new_element = new_item.add_nickname();
                    new_element.fill_from(nickname);
                }
                coappearances::VerticesData::Description(ref desc) => {
                    let mut new_element = new_item.add_description();
                    new_element.fill_from(desc);
                }
                coappearances::VerticesData::UnaryRelation(ref rel) => {
                    let mut new_element = new_item.add_unary_relation();
                    new_element.fill_from(rel);
                }
                coappearances::VerticesData::BinaryRelation(ref rel) => {
                    let mut new_element = new_item.add_binary_relation();
                    new_element.fill_from(rel);
                }
            }
        }
    }
    vertices_data.close().expect("close failed");

    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "vertices_data"
    ));
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "vertices_data_index"
    ));

    let mut chapters = flatdata::Vector::<coappearances::Chapter>::new();
    for ch in g.chapters().iter() {
        chapters.grow().fill_from(&ch);
    }

    gb.set_chapters(&chapters.as_view())
        .expect("set_chapters failed");
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "chapters"
    ));

    gb.set_strings(g.strings()).expect("set_strings failed");
    assert!(compare_resource(
        &source_archive_path,
        &archive_path,
        "strings"
    ));

    (archive_path, gb)
}

#[test]
fn read_write_coappearances() {
    copy_coappearances_archive(
        "tests/coappearances/karenina.archive",
        "read_write_coappearances/karenina.archive",
    );
}

#[test]
fn read_non_existent_statistics_subarchive() {
    let (archive_path, _) = copy_coappearances_archive(
        "tests/coappearances/karenina.archive",
        "read_non_existent_statistics_subarchive/karenina.archive",
    );

    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        archive_path,
    )));
    let g = coappearances::Graph::open(storage).expect("invalid archive");
    assert!(g.statistics().is_none());
}

#[test]
fn read_write_statistics_subarchive() {
    let (archive_path, mut gb) = copy_coappearances_archive(
        "tests/coappearances/karenina.archive",
        "read_write_statistics_subarchive/karenina.archive",
    );

    let mut builder = gb.statistics().expect("statistics failed");
    let mut inv = flatdata::StructBuf::<coappearances::Invariants>::new();
    inv.set_max_degree(71);
    inv.set_max_degree_ref(46);
    inv.set_min_degree(1);
    inv.set_min_degree_ref(9);
    inv.set_num_connected_components(1);
    builder.set_invariants(&inv).expect("set_invariants failed");

    let degrees = vec![
        4, 11, 25, 43, 3, 3, 4, 7, 2, 1, 1, 1, 12, 3, 1, 4, 8, 2, 6, 40, 2, 5, 2, 1, 1, 6, 3, 16,
        3, 5, 5, 1, 1, 1, 3, 3, 13, 44, 27, 5, 3, 10, 2, 11, 4, 6, 71, 7, 2, 5, 7, 6, 2, 11, 3, 1,
        1, 2, 3, 4, 6, 2, 2, 6, 5, 2, 3, 3, 1, 3, 1, 6, 3, 5, 3, 7, 1, 7, 4, 3, 2, 3, 1, 5, 1, 1,
        6, 10, 7, 6, 3, 1, 27, 18, 8, 3, 3, 2, 4, 2, 1, 10, 3, 4, 2, 9, 3, 6, 4, 1, 6, 50, 1, 15,
        2, 14, 1, 7, 1, 12, 15, 3, 3, 2, 1, 6, 15, 4, 7, 47, 6, 14, 3, 2, 1, 7, 10, 13,
    ];
    let mut vertex_degrees = builder
        .start_vertex_degrees()
        .expect("start_vertex_degrees failed");
    for deg in degrees {
        vertex_degrees.grow().expect("grow failed").set_value(deg)
    }
    vertex_degrees.close().expect("close failed");

    // compare
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        "tests/coappearances/karenina.archive".into(),
    )));
    let orig = coappearances::Graph::open(storage).expect("invalid archive");
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        archive_path,
    )));
    let copy = coappearances::Graph::open(storage).expect("invalid archive");

    let orig_stats = orig.statistics().as_ref().expect("orig statistics failed");
    let copy_stats = copy.statistics().as_ref().expect("copy statistics failed");

    assert_eq!(orig_stats.invariants(), copy_stats.invariants());
    assert_eq!(
        orig_stats.vertex_degrees().len(),
        copy_stats.vertex_degrees().len()
    );
    for i in 0..orig_stats.vertex_degrees().len() {
        assert_eq!(
            orig_stats.vertex_degrees().at(i),
            copy_stats.vertex_degrees().at(i)
        );
    }
}

#[test]
fn read_and_validate_calculate_data_subarchive() {
    let storage = Rc::new(RefCell::new(flatdata::FileResourceStorage::new(
        path::PathBuf::from("tests/coappearances/karenina.archive/statistics"),
    )));
    let stats = coappearances::Statistics::open(storage).expect("invalid archive");
    println!("{:?}", stats);

    let inv = stats.invariants();
    assert_eq!(inv.max_degree(), 71);
    assert_eq!(inv.max_degree_ref(), 46);
    assert_eq!(inv.min_degree(), 1);
    assert_eq!(inv.min_degree_ref(), 9);
    assert_eq!(inv.num_connected_components(), 1);

    let expected_degrees = vec![
        4, 11, 25, 43, 3, 3, 4, 7, 2, 1, 1, 1, 12, 3, 1, 4, 8, 2, 6, 40, 2, 5, 2, 1, 1, 6, 3, 16,
        3, 5, 5, 1, 1, 1, 3, 3, 13, 44, 27, 5, 3, 10, 2, 11, 4, 6, 71, 7, 2, 5, 7, 6, 2, 11, 3, 1,
        1, 2, 3, 4, 6, 2, 2, 6, 5, 2, 3, 3, 1, 3, 1, 6, 3, 5, 3, 7, 1, 7, 4, 3, 2, 3, 1, 5, 1, 1,
        6, 10, 7, 6, 3, 1, 27, 18, 8, 3, 3, 2, 4, 2, 1, 10, 3, 4, 2, 9, 3, 6, 4, 1, 6, 50, 1, 15,
        2, 14, 1, 7, 1, 12, 15, 3, 3, 2, 1, 6, 15, 4, 7, 47, 6, 14, 3, 2, 1, 7, 10, 13,
    ];
    for (index, deg) in stats.vertex_degrees().iter().enumerate() {
        assert_eq!(deg.value(), expected_degrees[index]);
    }
}

#![cfg(test)]

use std::path::{Path, PathBuf};
use std::{env, fs, io::Read, str};

pub mod coappearances;

fn read_and_validate_coappearances(
    storage: flatdata::StorageHandle,
) -> Result<(), std::str::Utf8Error> {
    let g = coappearances::Graph::open(storage).expect("invalid archive");
    println!("{:?}", g);

    let vertices = g.vertices();
    let edges = g.edges();
    assert_eq!(vertices.len(), 138);
    assert_eq!(edges.len(), 493);

    let meta = g.meta();
    assert_eq!(meta, g.meta());

    assert_eq!(
        g.strings().substring(meta.title_ref() as usize)?,
        "Anna Karenina (Анна Каренина)"
    );
    assert_eq!(
        g.strings().substring(meta.author_ref() as usize)?,
        "Leo Tolstoy (Лев Николаевич Толстой)"
    );

    let num_chapters = edges.iter().map(|e| e.count() as usize).sum();
    assert_eq!(g.chapters().len(), num_chapters);

    assert_eq!(
        g.strings().substring(vertices[0].name_ref() as usize)?,
        "Annushka"
    );
    assert_eq!(
        g.strings().substring(vertices[3].name_ref() as usize)?,
        "Anna Arkadyevna Karenina"
    );

    let e0 = &edges[0];
    assert_eq!(
        g.strings()
            .substring(vertices[e0.a_ref() as usize].name_ref() as usize)?,
        "Annushka"
    );
    assert_eq!(
        g.strings()
            .substring(vertices[e0.b_ref() as usize].name_ref() as usize)?,
        "Anna Arkadyevna Karenina"
    );

    let validate_chapters = |edge_ref: usize, expected| {
        let chapters_range = edges[edge_ref].chapters_range();
        let e_chapters: Vec<String> = g.chapters()
            [chapters_range.start as usize..chapters_range.end as usize]
            .iter()
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
    validate_chapters(edges.len() - 1, vec!["7.25"]);

    let vertices_data = g.vertices_data();
    assert_eq!(vertices_data.len(), vertices.len());

    let data: Vec<_> = vertices_data.at(0).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesDataRef::UnaryRelation(data) => {
            assert_eq!(g.strings().substring(data.kind_ref() as usize)?, "maid");
            assert_eq!(
                g.strings()
                    .substring(vertices[data.to_ref() as usize].name_ref() as usize)?,
                "Anna Arkadyevna Karenina"
            );
        }
        _ => unreachable!(),
    };

    let data: Vec<_> = vertices_data.at(1).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesDataRef::UnaryRelation(data) => {
            assert_eq!(
                g.strings().substring(data.kind_ref() as usize)?,
                "housekeeper"
            );
            assert_eq!(
                g.strings()
                    .substring(vertices[data.to_ref() as usize].name_ref() as usize)?,
                "Konstantin Dmitrievitch Levin"
            );
        }
        _ => unreachable!(),
    };

    let data: Vec<_> = vertices_data.at(vertices_data.len() - 1).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesDataRef::UnaryRelation(data) => {
            assert_eq!(
                g.strings().substring(data.kind_ref() as usize)?,
                "gambling friend"
            );
            assert_eq!(
                g.strings()
                    .substring(vertices[data.to_ref() as usize].name_ref() as usize)?,
                "Count Alexey Kirillovitch Vronsky"
            );
        }
        _ => unreachable!(),
    };
    Ok(())
}

#[test]
fn read_and_validate_coappearances_from_file_storage() -> Result<(), std::str::Utf8Error> {
    let storage = flatdata::FileResourceStorage::new("assets/karenina.archive");
    read_and_validate_coappearances(storage)
}

#[test]
#[cfg(feature = "tar")]
fn read_and_validate_coappearances_from_tar_archive_storage() -> Result<(), std::str::Utf8Error> {
    let storage = flatdata::TarArchiveResourceStorage::new("assets/karenina.tar")
        .expect("failed to read tar archive");
    read_and_validate_coappearances(storage)
}

fn check_files(name_a: &Path, name_b: &Path) {
    let mut fa = fs::File::open(name_a).unwrap();
    let mut buf_a = Vec::new();
    fa.read_to_end(&mut buf_a).unwrap();

    let mut fb = fs::File::open(name_b).unwrap();
    let mut buf_b = Vec::new();
    fb.read_to_end(&mut buf_b).unwrap();

    assert_eq!(
        buf_a,
        buf_b,
        "{}, vs {}",
        name_a.display(),
        name_b.display()
    );
}

fn check_resource(from: &Path, to: &Path, resource_name: &str) {
    check_files(&from.join(resource_name), &to.join(resource_name));
    check_files(
        &from.join(format!("{}.schema", resource_name)),
        &to.join(format!("{}.schema", resource_name)),
    );
}

fn copy_coappearances_archive(
    from_path: &str,
    to_path: &str,
) -> (PathBuf, coappearances::GraphBuilder) {
    // open for reading
    let source_archive_path = PathBuf::from(from_path);
    let storage = flatdata::FileResourceStorage::new(source_archive_path.clone());
    let g = coappearances::Graph::open(storage).expect("invalid archive");

    // open for writing
    let archive_path = env::temp_dir().join(to_path);
    println!("Will create archive in: {}", archive_path.to_str().unwrap());
    if archive_path.exists() {
        fs::remove_dir_all(&archive_path).expect("could not remove already existing archive");
    }
    fs::create_dir_all(&archive_path).expect("could not create archive dir");
    let storage = flatdata::FileResourceStorage::new(archive_path.clone());
    let gb = coappearances::GraphBuilder::new(storage).expect("could not create archive");

    // copy data
    let mut meta = coappearances::Meta::new();
    meta.fill_from(g.meta());
    gb.set_meta(&meta).expect("set_meta failed");
    check_resource(&source_archive_path, &archive_path, "meta");

    {
        let mut vertices = gb.start_vertices().expect("start_vertices failed");
        for v in g.vertices() {
            let w = vertices.grow().expect("grow failed");
            w.fill_from(v);
        }
        vertices.close().expect("close failed");
    }
    check_resource(&source_archive_path, &archive_path, "vertices");

    let mut edges = flatdata::Vector::<coappearances::Coappearance>::new();
    for e in g.edges() {
        edges.grow().fill_from(e);
    }
    // add final sentinel
    let sentinel = edges.grow();
    sentinel.set_first_chapter_ref(g.edges()[g.edges().len() - 1].chapters_range().end);
    sentinel.set_a_ref(std::u16::MAX as u32);
    sentinel.set_b_ref(std::u16::MAX as u32);
    gb.set_edges(edges.as_view()).expect("set_edges failed");

    check_resource(&source_archive_path, &archive_path, "edges");

    {
        let mut vertices_data = gb
            .start_vertices_data()
            .expect("start_vertices_data failed");
        for item in g.vertices_data().iter() {
            let mut new_item = vertices_data.grow().expect("grow failed");
            for element in item {
                match element {
                    coappearances::VerticesDataRef::Nickname(nickname) => {
                        let new_element = new_item.add_nickname();
                        new_element.fill_from(nickname);
                    }
                    coappearances::VerticesDataRef::Description(desc) => {
                        let new_element = new_item.add_description();
                        new_element.fill_from(desc);
                    }
                    coappearances::VerticesDataRef::UnaryRelation(rel) => {
                        let new_element = new_item.add_unary_relation();
                        new_element.fill_from(rel);
                    }
                    coappearances::VerticesDataRef::BinaryRelation(rel) => {
                        let new_element = new_item.add_binary_relation();
                        new_element.fill_from(rel);
                    }
                }
            }
        }
        vertices_data.close().expect("close failed");
    }

    check_resource(&source_archive_path, &archive_path, "vertices_data");
    check_resource(&source_archive_path, &archive_path, "vertices_data_index");

    let mut chapters = flatdata::Vector::<coappearances::Chapter>::new();
    for ch in g.chapters() {
        chapters.grow().fill_from(ch);
    }

    gb.set_chapters(chapters.as_view())
        .expect("set_chapters failed");
    check_resource(&source_archive_path, &archive_path, "chapters");

    gb.set_strings(&g.strings()).expect("set_strings failed");
    check_resource(&source_archive_path, &archive_path, "strings");

    (archive_path, gb)
}

#[test]
fn read_write_coappearances() {
    copy_coappearances_archive(
        "assets/karenina.archive",
        "read_write_coappearances/karenina.archive",
    );
}

#[test]
fn read_non_existent_statistics_subarchive() {
    let (archive_path, _) = copy_coappearances_archive(
        "assets/karenina.archive",
        "read_non_existent_statistics_subarchive/karenina.archive",
    );

    let storage = flatdata::FileResourceStorage::new(archive_path);
    let g = coappearances::Graph::open(storage).expect("invalid archive");
    assert!(g.statistics().is_none());
}

#[test]
fn read_write_statistics_subarchive() {
    let (archive_path, gb) = copy_coappearances_archive(
        "assets/karenina.archive",
        "read_write_statistics_subarchive/karenina.archive",
    );

    let builder = gb.statistics().expect("statistics failed");
    let mut inv = coappearances::Invariants::new();
    {
        inv.set_max_degree(71);
        inv.set_max_degree_ref(46);
        inv.set_min_degree(1);
        inv.set_min_degree_ref(9);
        inv.set_num_connected_components(1);
    }
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
    let storage = flatdata::FileResourceStorage::new("assets/karenina.archive");
    let orig = coappearances::Graph::open(storage).expect("invalid archive");
    let storage = flatdata::FileResourceStorage::new(archive_path);
    let copy = coappearances::Graph::open(storage).expect("invalid archive");

    let orig_stats = orig.statistics().expect("orig statistics failed");
    let copy_stats = copy.statistics().expect("copy statistics failed");

    assert_eq!(orig_stats.invariants(), copy_stats.invariants());
    assert_eq!(
        orig_stats.vertex_degrees().len(),
        copy_stats.vertex_degrees().len()
    );
    for i in 0..orig_stats.vertex_degrees().len() {
        assert_eq!(
            orig_stats.vertex_degrees()[i],
            copy_stats.vertex_degrees()[i]
        );
    }
}

#[test]
fn read_and_validate_calculate_data_subarchive() {
    let storage =
        flatdata::FileResourceStorage::new(PathBuf::from("assets/karenina.archive/statistics"));
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

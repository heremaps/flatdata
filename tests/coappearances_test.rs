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
    println!("{:?}", g);

    let vertices = g.vertices();
    let edges = g.edges();
    assert_eq!(vertices.size(), 138);
    assert_eq!(edges.size(), 494);

    // Note: We could use `from_utf8_unchecked` here, which simply does a raw pointer conversion,
    // however we use the safe version of the function, which does exactly the same except it
    // validates that the string is utf8. For that, it need to scan the whole string once, which is
    // usually undesirable for huge data.
    let strings = str::from_utf8(g.strings()).expect("invalid utf8 string");

    let meta = g.meta().clone();
    assert_eq!(&meta, g.meta());

    assert_eq!(
        substring(strings, meta.title_ref()),
        "Anna Karenina (Анна Каренина)"
    );
    assert_eq!(
        substring(strings, meta.author_ref()),
        "Leo Tolstoy (Лев Николаевич Толстой)"
    );

    let num_chapters = edges.iter().map(|e| e.count() as usize).sum();
    assert_eq!(g.chapters().size(), num_chapters);

    assert_eq!(substring(strings, vertices.index(0).name_ref()), "Annushka");
    assert_eq!(
        substring(strings, vertices.index(3).name_ref()),
        "Anna Arkadyevna Karenina"
    );

    let e0 = edges.index(0);
    assert_eq!(
        substring(strings, vertices.index(e0.a_ref() as usize).name_ref()),
        "Annushka"
    );
    assert_eq!(
        substring(strings, vertices.index(e0.b_ref() as usize).name_ref()),
        "Anna Arkadyevna Karenina"
    );

    let validate_chapters = |edge_ref, expected| {
        let e = edges.index(edge_ref);
        let chapters_start = e.first_chapter_ref() as usize;
        let chapters_end = edges.index(edge_ref + 1).first_chapter_ref() as usize;
        let e_chapters: Vec<String> = g.chapters()
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
            "1.29", "2.27", "3.15", "3.16", "6.19", "7.25", "7.27", "7.29"
        ],
    );
    validate_chapters(1, vec!["6.19"]);
    // Note: Last element in edges is not an edge but a sentinel which allows us to calculate the
    // range of chapter.
    validate_chapters(edges.size() - 2, vec!["7.25"]);

    let vertices_data = g.vertices_data();
    assert_eq!(vertices_data.size(), vertices.size());

    let data: Vec<_> = vertices_data.index(0).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "maid");
            assert_eq!(
                substring(strings, vertices.index(data.to_ref() as usize).name_ref()),
                "Anna Arkadyevna Karenina"
            );
        }
        _ => assert!(false),
    };

    let data: Vec<_> = vertices_data.index(1).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "housekeeper");
            assert_eq!(
                substring(strings, vertices.index(data.to_ref() as usize).name_ref()),
                "Konstantin Dmitrievitch Levin"
            );
        }
        _ => assert!(false),
    };

    let data: Vec<_> = vertices_data.index(vertices_data.size() - 1).collect();
    assert_eq!(data.len(), 1);
    match data[0] {
        coappearances::VerticesData::UnaryRelation(ref data) => {
            assert_eq!(substring(strings, data.kind_ref()), "gambling friend");
            assert_eq!(
                substring(strings, vertices.index(data.to_ref() as usize).name_ref()),
                "Count Alexey Kirillovitch Vronsky"
            );
        }
        _ => assert!(false),
    };
}

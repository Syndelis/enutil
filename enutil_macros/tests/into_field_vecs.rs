use enutil::IntoFieldVecs;
use enutil_macros::IntoFieldVecs;

#[test]
fn base_test() {
    #[derive(IntoFieldVecs)]
    struct Entry {
        id: i32,
        name: String,
    }

    let entries = vec![
        Entry {
            id: 0,
            name: "First".to_string(),
        },
        Entry {
            id: 1,
            name: "Second".to_string(),
        },
    ];

    let (ids, names) = IntoFieldVecs::into_field_vecs(entries);
    assert_eq!(&ids, &[0, 1]);
    assert_eq!(&names, &["First", "Second"]);
}

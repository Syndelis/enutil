use enutil::IntoInsertionArrays;
use enutil_macros::IntoInsertionArrays;

#[test]
fn base_test() {
    #[derive(IntoInsertionArrays)]
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

    let (ids, names) = IntoInsertionArrays::into_insertion_arrays(entries);
    assert_eq!(&ids, &[0, 1]);
    assert_eq!(&names, &["First", "Second"]);
}

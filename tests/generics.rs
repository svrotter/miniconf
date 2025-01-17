use miniconf::{Miniconf, MiniconfAtomic};
use serde::{Deserialize, Serialize};

#[test]
fn generic_type() {
    #[derive(Miniconf, Default)]
    struct Settings<T: Miniconf> {
        pub data: T,
    }

    let mut settings = Settings::<f32>::default();
    settings
        .string_set("data".split('/').peekable(), b"3.0")
        .unwrap();
    assert_eq!(settings.data, 3.0);

    // Test metadata
    let metadata = settings.get_metadata();
    assert_eq!(metadata.max_depth, 2);
    assert_eq!(metadata.max_topic_size, "data".len());
}

#[test]
fn generic_array() {
    #[derive(Miniconf, Default)]
    struct Settings<T: Miniconf> {
        pub data: [T; 2],
    }

    let mut settings = Settings::<f32>::default();
    settings
        .string_set("data/0".split('/').peekable(), b"3.0")
        .unwrap();

    assert_eq!(settings.data[0], 3.0);

    // Test metadata
    let metadata = settings.get_metadata();
    assert_eq!(metadata.max_depth, 3);
    assert_eq!(metadata.max_topic_size, "data/0".len());
}

#[test]
fn generic_struct() {
    #[derive(Miniconf, Default)]
    struct Settings<T> {
        pub inner: T,
    }

    #[derive(MiniconfAtomic, Deserialize, Serialize, Default)]
    struct Inner {
        pub data: f32,
    }

    let mut settings = Settings::<Inner>::default();
    settings
        .string_set("inner".split('/').peekable(), b"{\"data\": 3.0}")
        .unwrap();

    assert_eq!(settings.inner.data, 3.0);

    // Test metadata
    let metadata = settings.get_metadata();
    assert_eq!(metadata.max_depth, 2);
    assert_eq!(metadata.max_topic_size, "inner".len());
}

#[test]
fn generic_atomic() {
    #[derive(Miniconf, Default)]
    struct Settings<T> {
        pub atomic: Inner<T>,
    }

    #[derive(Deserialize, Serialize, MiniconfAtomic, Default)]
    struct Inner<T> {
        pub inner: [T; 5],
    }

    let mut settings = Settings::<f32>::default();
    settings
        .string_set(
            "atomic".split('/').peekable(),
            b"{\"inner\": [3.0, 0, 0, 0, 0]}",
        )
        .unwrap();

    assert_eq!(settings.atomic.inner[0], 3.0);

    // Test metadata
    let metadata = settings.get_metadata();
    assert_eq!(metadata.max_depth, 2);
    assert_eq!(metadata.max_topic_size, "atomic".len());
}

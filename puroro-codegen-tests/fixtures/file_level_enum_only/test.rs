//! File-level enum generation without any message types.

use crate::file_level_enum_only::demo::Color;

#[test]
fn enum_constants_and_default() {
    assert_eq!(Color::UNSPECIFIED, Color::default());
    assert_eq!(Color::RED, Color::from(1));
    assert_eq!(Color::GREEN, Color::from(2));
    assert_eq!(i32::try_from(Color::RED).unwrap(), 1);
}

#[test]
fn unknown_open_value() {
    let unknown = Color::from(99);
    assert_eq!(i32::try_from(unknown), Err(99));
}

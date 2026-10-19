//! Two-hop inlined `School → Student → Point`.

use ::puroro::{Message, StringMut};
use ::puroro_rt::INLINE_CAP;
use ::puroro_rt::Varint;
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_sample_generated::school::{BIT_NAME, FIELD_STUDENT};
use ::puroro_sample_generated::student::FIELD_LOCATION;
use ::puroro_sample_generated::{Address, Point, School, Student};

/// Appends `v` as a base-128 varint.
fn encode_u64_varint(mut v: u64, buf: &mut Vec<u8>) {
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if v == 0 {
            break;
        }
    }
}

fn decode_u64_varint_at(bytes: &[u8], i: usize) -> Option<(u64, usize)> {
    let mut v = 0u64;
    let mut shift = 0;
    for (n, &b) in bytes[i..].iter().enumerate() {
        v |= u64::from(b & 0x7f) << shift;
        if b & 0x80 == 0 {
            return Some((v, n + 1));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
    None
}

/// Returns the LEN payload of the first occurrence of `field_number` (wire type 2).
fn find_len_field(bytes: &[u8], field_number: u32) -> Option<&[u8]> {
    let mut i = 0;
    while i < bytes.len() {
        let (tag, n) = decode_u64_varint_at(bytes, i)?;
        i += n;
        let num = (tag >> 3) as u32;
        let wt = tag & 7;
        match wt {
            0 => {
                let (_, n) = decode_u64_varint_at(bytes, i)?;
                i += n;
            }
            2 => {
                let (len, n) = decode_u64_varint_at(bytes, i)?;
                i += n;
                let end = i + usize::try_from(len).ok()?;
                if end > bytes.len() {
                    return None;
                }
                let payload = &bytes[i..end];
                i = end;
                if num == field_number {
                    return Some(payload);
                }
            }
            _ => return None,
        }
    }
    None
}

fn contains_varint_field(bytes: &[u8], field_number: u32, value: u64) -> bool {
    let mut encoded = Vec::new();
    encode_u64_varint(u64::from(field_number) << 3, &mut encoded);
    encode_u64_varint(value, &mut encoded);
    bytes
        .windows(encoded.len())
        .any(|w| w == encoded.as_slice())
}

fn top_level_has_varint_field(bytes: &[u8], field_number: u32) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let Some((tag, n)) = decode_u64_varint_at(bytes, i) else {
            return false;
        };
        i += n;
        let num = (tag >> 3) as u32;
        let wt = tag & 7;
        match wt {
            0 => {
                let Some((_, n)) = decode_u64_varint_at(bytes, i) else {
                    return false;
                };
                i += n;
                if num == field_number {
                    return true;
                }
            }
            2 => {
                let Some((len, n)) = decode_u64_varint_at(bytes, i) else {
                    return false;
                };
                i += n + usize::try_from(len).unwrap_or(0);
            }
            _ => return false,
        }
    }
    false
}

#[test]
fn owned_student_roundtrip() {
    let mut student = Student::new();
    *student.year_mut() = 2024;
    *student.location_mut().x_mut() = 3;
    *student.location_mut().y_mut() = 7;

    let decoded: Student = Student::decode(&student.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.year().get(), 2024);
    let loc = decoded.location().expect("location");
    assert_eq!(loc.x(), 3);
    assert_eq!(loc.y(), 7);

    let cloned = student.clone();
    assert_eq!(student, cloned);
}

#[test]
fn inlined_student_unset_omits_from_wire() {
    let school = School::new();
    assert!(school.student().is_none());
    let decoded: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    assert!(decoded.student().is_none());
}

#[test]
fn two_hop_roundtrip_and_assign() {
    let mut school = School::new();
    school.name_mut().set("North");
    *school.student_mut().year_mut() = 2024;
    *school.student_mut().location_mut().x_mut() = 3;
    *school.student_mut().location_mut().y_mut() = 7;

    let decoded: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.name().get(), "North");
    let student = decoded.student().expect("student");
    assert_eq!(student.year().get(), 2024);
    let loc = student.location().expect("location");
    assert_eq!(loc.x(), 3);
    assert_eq!(loc.y(), 7);

    let mut src = Student::new();
    *src.year_mut() = 1999;
    let mut p = Point::new();
    *p.x_mut() = 11;
    *p.y_mut() = 13;
    src.set_location(p);
    school.set_student(src);
    assert_eq!(school.student().unwrap().year().get(), 1999);
    assert_eq!(school.student().unwrap().location().unwrap().x(), 11);
    assert_eq!(school.student().unwrap().location().unwrap().y(), 13);
}

/// Student's `year` presence lives on `Student`, not on `School.name`.
#[test]
fn student_year_does_not_set_school_name_bit() {
    assert_eq!(BIT_NAME, 0);

    let mut school = School::new();
    assert!(!school.name().is_set());
    *school.student_mut().year_mut() = 2024;

    assert!(
        !school.name().is_set(),
        "year presence must not land on School's name bit"
    );
    assert_eq!(school.student().unwrap().year().get(), 2024);

    let decoded: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    assert!(!decoded.name().is_set());
    assert_eq!(decoded.student().unwrap().year().get(), 2024);
}

#[test]
fn two_hop_merge_and_clear() {
    let mut school = School::new();
    *school.student_mut().year_mut() = 1;
    *school.student_mut().location_mut().x_mut() = 10;

    let mut other = School::new();
    *other.student_mut().location_mut().y_mut() = 20;

    school.merge_from(&mut &other.encode_to_vec()[..]).unwrap();
    let student = school.student().unwrap();
    assert_eq!(student.year().get(), 1);
    let loc = student.location().expect("location");
    assert_eq!(loc.x(), 10);
    assert_eq!(loc.y(), 20);

    school.clear_student();
    assert!(school.student().is_none());
    let decoded: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    assert!(decoded.student().is_none());
}

#[test]
fn location_unknown_stays_inside_nested_len() {
    // School { student { location { x=1, unknown 99=5 } } }
    let mut location_payload = Vec::new();
    encode_varint_field(
        field_number_const::<1>(),
        Varint::from_uint64(1),
        &mut location_payload,
    );
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut location_payload,
    );

    let mut student_payload = Vec::new();
    encode_u64_varint(u64::from(FIELD_LOCATION) << 3 | 2, &mut student_payload);
    encode_u64_varint(location_payload.len() as u64, &mut student_payload);
    student_payload.extend_from_slice(&location_payload);

    let mut bytes = Vec::new();
    encode_u64_varint(u64::from(FIELD_STUDENT) << 3 | 2, &mut bytes);
    encode_u64_varint(student_payload.len() as u64, &mut bytes);
    bytes.extend_from_slice(&student_payload);

    let mut school = School::new();
    school.merge_from(&mut &bytes[..]).unwrap();
    assert!(school.unknown_fields().next().is_none());
    let student = school.student().expect("student");
    assert_eq!(student.location().expect("location").x(), 1);

    let encoded = school.encode_to_vec();
    let student_len = find_len_field(&encoded, FIELD_STUDENT).expect("student LEN");
    let location_len = find_len_field(student_len, FIELD_LOCATION).expect("location LEN");
    assert!(
        contains_varint_field(location_len, 99, 5),
        "unknown tag 99 must round-trip inside location LEN, got {location_len:?}"
    );
    assert!(
        !top_level_has_varint_field(&encoded, 99),
        "location unknown must not surface as a top-level School field"
    );
    assert!(
        !top_level_has_varint_field(student_len, 99),
        "location unknown must not surface as a Student-level field"
    );

    school.clear_student();
    assert!(school.student().is_none());
    assert!(school.unknown_fields().next().is_none());
    let cleared = school.encode_to_vec();
    assert!(find_len_field(&cleared, FIELD_STUDENT).is_none());
    assert!(!top_level_has_varint_field(&cleared, 99));
}

#[test]
fn school_clone_and_eq() {
    let mut school = School::new();
    school.name_mut().set("East");
    *school.student_mut().year_mut() = 2020;
    *school.student_mut().location_mut().x_mut() = 1;

    let mut cloned = school.clone();
    assert_eq!(school, cloned);
    *cloned.student_mut().year_mut() = 2021;
    assert_ne!(school, cloned);
}

#[test]
fn inlined_home_heap_string_clone_and_clear() {
    let heap_street = "S".repeat(INLINE_CAP + 8);
    let mut school = School::new();
    school
        .student_mut()
        .home_mut()
        .street_mut()
        .set(&heap_street);

    assert!(
        !school.name().is_set(),
        "Address SSO heap bit must not land on School name"
    );
    {
        let student = school.student().unwrap();
        let home = student.home().unwrap();
        assert_eq!(home.street().get(), heap_street);
    }

    let cloned = school.clone();
    {
        let cloned_student = cloned.student().unwrap();
        let cloned_home = cloned_student.home().unwrap();
        assert_eq!(cloned_home.street().get(), heap_street);
        assert!(!cloned.name().is_set());
    }

    let decoded: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    {
        let decoded_student = decoded.student().unwrap();
        let decoded_home = decoded_student.home().unwrap();
        assert_eq!(decoded_home.street().get(), heap_street);
    }

    school.clear_student();
    assert!(school.student().is_none());
    let after_clear: School = School::decode(&school.encode_to_vec()[..]).expect("decode");
    assert!(after_clear.student().is_none());

    let mut owned = Student::new();
    let mut addr = Address::new();
    addr.street_mut().set(&heap_street);
    addr.city_mut().set("Kyoto");
    owned.set_home(addr);
    school.set_student(owned);
    {
        let assigned = school.student().unwrap();
        let assigned_home = assigned.home().unwrap();
        assert_eq!(assigned_home.street().get(), heap_street);
        assert_eq!(assigned_home.city().get(), "Kyoto");
    }
}

fn year_of(s: &Student) -> i32 {
    s.year().get()
}

#[test]
fn owned_and_inlined_student_share_the_same_type() {
    let mut owned = Student::new();
    *owned.year_mut() = 11;
    assert_eq!(year_of(&owned), 11);

    let mut school = School::new();
    *school.student_mut().year_mut() = 12;
    assert_eq!(year_of(school.student().unwrap()), 12);
}

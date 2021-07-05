#![allow(unused)]

use sample_pb;

#[cfg(test)]
mod tests {
    use puroro::DeserFromBytesIter;

    use super::*;
    use std::default::Default;

    const TEST1_INPUT: &[u8] = &[0x08, 0x96, 0x01];
    const TEST2_INPUT: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];

    const TEST1_EXPECTED: i32 = 150;
    const TEST2_EXPECTED: &str = "testing";

    #[test]
    fn test1_simple() {
        use std::io::Read as _;
        let mut t1 = <sample_pb::sample2::Test1 as Default>::default();
        t1.deser(TEST1_INPUT.bytes()).unwrap();
        assert_eq!(Some(TEST1_EXPECTED), t1.a);
    }

    #[test]
    fn test2_simple() {
        use std::io::Read as _;
        use std::ops::Deref as _;
        let mut t2 = <sample_pb::sample2::Test2 as Default>::default();
        t2.deser(TEST2_INPUT.bytes()).unwrap();
        assert_eq!(Some(TEST2_EXPECTED.into()), t2.b);
    }
}

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod code_generator_response {
#[derive(Debug, Clone)]
pub enum Feature {
    FeatureNone = 0,
    FeatureProto3Optional = 1,
}
impl ::std::convert::TryFrom<i32> for Feature {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            0 => Ok(Self::FeatureNone),
            1 => Ok(Self::FeatureProto3Optional),
            x => Err(x),
        }
    }
}
impl ::std::convert::From<Feature> for i32 {
    fn from(val: Feature) -> i32 {
        val as i32
    }
}
} // mod code_generator_response

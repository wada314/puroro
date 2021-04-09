mod code_generator_response {
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl std::convert::TryFrom<i32> for Feature {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                x => Err(x),
            }
        }
    }
}

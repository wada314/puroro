use ::num_derive::FromPrimitive;
#[non_exhaustive]
#[derive(FromPrimitive, Debug)]
pub enum WireType {
    Variant = 0,
    Bytes64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bytes32 = 5,
}

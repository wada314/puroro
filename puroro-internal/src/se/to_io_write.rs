use crate::ErrorKind;
use crate::Result;
use ::puroro::types::WireType;
use ::puroro::variant::Variant;
use ::std::convert::TryInto;
use ::std::io::Write;

pub fn write_field_number_and_wire_type<W: Write>(
    out: &mut W,
    field_number: i32,
    wire_type: WireType,
) -> Result<()> {
    let encoding_val = <i32 as TryInto<u32>>::try_into(field_number)
        .map_err(|_| ErrorKind::InvalidFieldNumber)?
        .checked_shl(3)
        .ok_or(ErrorKind::InvalidFieldNumber)?
        | (wire_type as u32);
    let variant = Variant::from_u32(encoding_val)?;
    variant.encode_bytes(out)?;
    Ok(())
}

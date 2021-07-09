use crate::ErrorKind;
use crate::Result;
use ::puroro::types::WireType;
use ::puroro::variant::Variant;
use ::std::io::Write;

pub fn write_field_number_and_wire_type<W: Write>(
    out: &mut W,
    field_number: i32,
    wire_type: WireType,
) -> Result<()> {
    let encoding_val = field_number
        .checked_shl(3)
        .ok_or(ErrorKind::InvalidFieldNumber)?
        | (wire_type as i32);
    let variant = Variant::from_i32(encoding_val)?;
    variant.encode_bytes(out)?;
    Ok(())
}

use crate::Result;
use ::puroro::types::WireType;
use ::std::io::Write;

pub fn write_field_number_and_wire_type<W: Write>(
    out: &mut W,
    field_number: i32,
    wire_type: WireType,
) -> Result<()> {
    todo!();
}

pub mod de;
pub mod impls;
pub mod se;

use crate::desc::{FieldDescriptor, MessageDescriptor};
use crate::once_cell::sync::Lazy;

pub fn init_message_descriptor(lazy_fields: Lazy<&'static [FieldDescriptor]>) -> MessageDescriptor {
    MessageDescriptor { lazy_fields }
}

pub fn init_field_descriptor(
    lazy_containing_type: Lazy<&'static MessageDescriptor>,
) -> FieldDescriptor {
    FieldDescriptor {
        lazy_containing_type,
    }
}

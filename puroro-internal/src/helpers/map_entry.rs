use crate::deser::BytesIter;
use crate::deser::DeserializableMessageFromIter;
use crate::helpers::deser_field::DeserializableFromIterField;
use crate::tags;
use crate::tags::FieldTypeTag;
use crate::types::FieldData;
use crate::{ErrorKind, Result};
use std::marker::PhantomData;

pub struct MapEntry<KT, VT, KR, VR>
where
    KT: FieldTypeTag,
    VT: FieldTypeTag,
{
    key: KR,
    value: VR,
    _phantom: PhantomData<(KT, VT)>,
}

impl<KT, VT, KR, VR> MapEntry<KT, VT, KR, VR>
where
    KT: FieldTypeTag,
    VT: FieldTypeTag,
{
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> (KR, VR),
    {
        let (key, value) = (f)();
        Self {
            key,
            value,
            _phantom: PhantomData,
        }
    }
}

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

impl<KT, VT, KR, VR> DeserializableMessageFromIter for MapEntry<KT, VT, KR, VR>
where
    KT: FieldTypeTag,
    VT: FieldTypeTag,
    KR: DeserializableFromIterField<(KT, tags::Required)>,
    VR: DeserializableFromIterField<(VT, tags::Required)>,
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: FieldData<&'a mut BytesIter<'b, I>>,
        field_number: usize,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        match field_number {
            0 => <KR as DeserializableFromIterField<(KT, tags::Required)>>::deser(
                &mut self.key,
                field,
                f,
            )?,
            1 => <VR as DeserializableFromIterField<(VT, tags::Required)>>::deser(
                &mut self.value,
                field,
                f,
            )?,
            _ => Err(ErrorKind::UnexpectedFieldId)?,
        }
        todo!()
    }
}

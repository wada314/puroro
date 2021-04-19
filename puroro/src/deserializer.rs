use crate::types::Field;

use self::stream::LengthDelimitedDeserializer;
use crate::Result;

pub mod stream;

pub trait Deserializer {
    fn deserialize<H: MessageDeserializeEventHandler>(self, handler: H) -> Result<H::Target>;
}

pub trait MessageDeserializeEventHandler {
    type Target;
    fn finish(self) -> Result<Self::Target>;

    fn met_field<T: LengthDelimitedDeserializer>(
        &mut self,
        field: Field<T>,
        field_number: usize,
    ) -> Result<()>;
}

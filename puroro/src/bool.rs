pub struct True();
pub struct False();

pub trait BoolTypes {}
impl BoolTypes for True {}
impl BoolTypes for False {}

use std::marker::PhantomData;
pub struct MarkedAt(PhantomData<i64>);
impl MarkedAt {
    pub const VALUE_FOR_INDICATION_OF_MARK_ABSENCE: u8 = 0;
}
pub struct CreatedAt(PhantomData<i64>);

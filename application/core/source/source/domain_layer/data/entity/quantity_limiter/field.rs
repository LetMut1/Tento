use std::marker::PhantomData;
pub struct OwnedChannelsQuantity(PhantomData<u8>);
impl OwnedChannelsQuantity {
    pub const MAXIMUM_VALUE: u8 = 30;
}

pub struct CreatedAt(PhantomData<i64>);

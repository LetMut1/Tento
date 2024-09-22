use super::channel::Channel_Id;
use std::marker::PhantomData;
pub struct ChannelInnerLink {
    pub from: i64,
    _from: PhantomData<Channel_Id>,

    pub to: i64,
    _to: PhantomData<Channel_Id>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
    pub fn new(from: i64, to: i64, created_at: i64) -> Self {
        return Self {
            from,
            _from: PhantomData,
            to,
            _to: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
pub struct CreatedAt;

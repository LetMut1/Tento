use std::marker::PhantomData;
use super::channel::Channel_Id;

pub use self::CreatedAt as ChannelInnerLink_CreatedAt;

pub struct ChannelInnerLink {
    from: i64,
    _from: PhantomData<Channel_Id>,

    to: i64,
    _to: PhantomData<Channel_Id>,

    created_at: String,
    _created_at: PhantomData<CreatedAt>
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;

    pub fn new(
        from: i64,
        to: i64,
        created_at: String
    ) -> Self {
        return Self {
            from,
            _from: PhantomData,
            to,
            _to: PhantomData,
            created_at,
            _created_at: PhantomData
        };
    }
}

pub struct CreatedAt;
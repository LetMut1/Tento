use super::channel::Channel_Id;
use std::marker::PhantomData;

pub use self::CreatedAt as ChannelInnerLink_CreatedAt;

pub struct ChannelInnerLink {
    pub from: i64,
    _from: PhantomData<Channel_Id>,

    pub to: i64,
    _to: PhantomData<Channel_Id>,

    pub created_at: CreatedAt,
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;

    pub fn new(
        from: i64,
        to: i64,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            from,
            _from: PhantomData,
            to,
            _to: PhantomData,
            created_at,
        };
    }
}

pub struct CreatedAt(pub String);